//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1181;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1182;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1183;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1184;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1185;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1186;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta189(t1041: f64, t4571: f64, t1616: f64, t884: f64, t3071: f64, t1023: f64, t1539: f64, t247: f64, t375: f64, t1043: f64, t2775: f64, t3961: f64, t2770: f64, t3061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4572, t4574, t4575) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1181(t1041, t4571, t1616, t884, t3071);
        let (t4578, t4579) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1182(t1023, t1539, t3071);
        let t4582 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1183(t247, t375);
        let t4583 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1184(t1043, t2775);
        let (t4584, t4585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1185(t3961, t4583, t4582);
        let t4588 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1186(t2770, t3061);
        let (t4589, t4590) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1187(t3961, t4588, t4582);
    (t4572, t4574, t4575, t4578, t4579, t4582, t4583, t4584, t4585, t4588, t4589, t4590)
}
