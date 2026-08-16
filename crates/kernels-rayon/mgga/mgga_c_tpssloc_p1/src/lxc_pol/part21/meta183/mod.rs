//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1156;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1157;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1158;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1159;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1160;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta183(t4395: f64, t913: f64, t893: f64, t1556: f64, t2844: f64, t912: f64, t2842: f64, t2766: f64, t2848: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t1561: f64, t923: f64, t1569: f64, t931: f64, t2824: f64, t2868: f64, t2875: f64, t4363: f64, t4371: f64, t4379: f64, t4381: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t932: f64, t1568: f64, t2888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4396 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1156(t4395, t913);
        let (t4398, t4399) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1157(t4396, t893, t1556, t2844);
        let (t4400, t4402, t4408, t4411) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1158(t4399, t912, t2842, t2766, t2848, t4335, t4340, t4345, t4349, t1561, t923);
        let (t4416, t4433) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1159(t1569, t931, t2766, t2824, t2868, t2875, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387, t4390, t4393);
        let t4434 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1160(t4433, t932);
        let t4437 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1161(t1568, t2888);
    (t4396, t4398, t4399, t4400, t4402, t4408, t4411, t4416, t4433, t4434, t4437)
}
