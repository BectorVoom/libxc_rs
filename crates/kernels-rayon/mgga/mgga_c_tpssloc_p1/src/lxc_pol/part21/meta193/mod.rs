//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1208;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1209;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1210;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1211;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta193(t1060: f64, t4677: f64, t381: f64, t4649: f64, t1022: f64, t1932: f64, t360: f64, t1629: f64, t1625: f64, t383: f64, t4657: f64, t1003: f64, t1058: f64, t1061: f64, t1063: f64, t1610: f64, t1630: f64, t1632: f64, t3180: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4674: f64, t1055: f64, t1052: f64, t1066: f64, t1635: f64, t3026: f64, t3169: f64, t388: f64, t4553: f64, t4555: f64, t4557: f64, t4559: f64, t4658: f64, t4660: f64, t4665: f64, t193: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4678, t4680, t4681, t4684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1208(t1060, t4677, t381, t4649, t1022, t1932, t360);
        let (t4685, t4689, t4691, t4693) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1209(t1629, t4684, t1022, t1625, t1060, t383, t4657, t1003, t1058, t1061, t1063, t1610, t1630, t1632, t3180, t3186, t3200, t353, t384, t4615, t4669, t4674, t4678, t4681);
        let t4694 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1210(t1055, t4693);
        let t4696 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1211(t1052, t1066, t1635, t3026, t3169, t388, t4553, t4555, t4557, t4559, t4658, t4660, t4665, t4694);
        let t4700 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1212(t193, t336);
    (t4678, t4680, t4681, t4684, t4685, t4689, t4691, t4693, t4694, t4696, t4700)
}
