//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 289/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk289(t453: f64, t8: f64, t6: f64, t95: f64, t1194: f64, t1199: f64, t1201: f64, t408: f64, t101: f64, t1088: f64, t1093: f64, t1094: f64, t1121: f64, t1189: f64, t387: f64, t397: f64, t400: f64, t415: f64, t72: f64, t75: f64) -> (f64, f64, f64) {
    let t1204 = 1.0_f64 / t8 / t453;
    let t1205 = t6 * t1204;
    let t1206 = t95 * t1205;
    let t1208 = -0.11955719325063177623e-1_f64 * t408 + 0.40985e-2_f64 * t1194 - 0.10566666666666666667e-2_f64 * t1199 + 0.3884654180847230157e-4_f64 * t1201 - 0.420109375e-5_f64 * t1206;
    let t1210 = 0.23426533963880895498e-2_f64 * t408 * t72 + 0.46853067927761790996e-2_f64 * t1088 * t397 + 0.70279601891642686494e-2_f64 * t1093 * t1094 - 0.23426533963880895498e-2_f64 * t387 * t1121 - t1189 * t101 - 2.0_f64 * t400 * t415 - t75 * t1208;
    (t1206, t1208, t1210)
}
