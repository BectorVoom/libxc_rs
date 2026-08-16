//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 979/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk979(t6967: f64, t6969: f64, t6972: f64, t9008: f64, t9012: f64, t9029: f64, t387: f64, t9011: f64, t7183: f64, t1434: f64, t2578: f64, t1422: f64, t2539: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9031 = -t6967 + 0.24722222222222222222e-1_f64 * t6969 - 0.92708333333333333333e-2_f64 * t6972 + 0.12361111111111111111e-1_f64 * t9008 - t9012 + 0.278125e-1_f64 * t9029;
    let t9032 = t9031 * t387;
    let t9037 = 0.34246666666666666666e-1_f64 * t9011;
    let t9039 = -t7183 + 0.45662222222222222222e-1_f64 * t6969 - 0.17123333333333333333e-1_f64 * t6972 + 0.22831111111111111111e-1_f64 * t9008 - t9037 + 0.5137e-1_f64 * t9029;
    let t9042 = t1434 * t2578;
    let t9045 = t1422 * t2539;
    (t9031, t9032, t9037, t9039, t9042, t9045)
}
