//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3132/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132(t12855: f64, t12916: f64, t17455: f64, t3584: f64, t5333: f64, t1222: f64, t16738: f64, t17240: f64, t16742: f64, t16733: f64, t13036: f64, t13039: f64, t57403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57735 = t12855 * t12916 * t17455;
    let t57737 = t5333 * t3584;
    let t57743 = t1222 * t17240 * t16738;
    let t57746 = t1222 * t17240 * t16742;
    let t57749 = t1222 * t17240 * t16733;
    let t57759 = t13036 * t13039 * t57403;
    (t57735, t57737, t57743, t57746, t57749, t57759)
}
