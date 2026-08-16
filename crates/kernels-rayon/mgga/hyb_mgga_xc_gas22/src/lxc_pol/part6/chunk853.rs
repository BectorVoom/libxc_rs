//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 853/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk853(t2274: f64, t270: f64, t2289: f64, t835: f64, t2250: f64, t816: f64, t2282: f64, t839: f64, t2306: f64, t2314: f64, t6527: f64, t260: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6712 = 1.0_f64 / t2274 / t270;
    let t6716 = t835 * t2289;
    let t6722 = t816 * t2250;
    let t6729 = t2282 * t839;
    let t6737 = t2306 * t2314;
    let t6749 = 0.53272592592592592592e-1_f64 * t6527;
    let t6759 = t260 * t2282;
    (t6712, t6716, t6722, t6729, t6737, t6749, t6759)
}
