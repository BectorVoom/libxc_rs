//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1083/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1083(t10534: f64, t10549: f64, t6530: f64, t6552: f64, t8676: f64, t8751: f64, t284: f64, t4108: f64, t787: f64, t811: f64, t1347: f64, t8865: f64) -> (f64, f64, f64, f64, f64) {
    let t10551 = -t6552 + 0.12361111111111111111e-1_f64 * t6530 + 0.24722222222222222223e-1_f64 * t8676 - t8751 - 0.92708333333333333333e-2_f64 * t10534 + 0.278125e-1_f64 * t10549;
    let t10552 = t10551 * t284;
    let t10555 = t4108 * t787;
    let t10557 = 1.0_f64 * t10555 * t811;
    let t10559 = 2.0_f64 * t8865 * t1347;
    (t10551, t10552, t10555, t10557, t10559)
}
