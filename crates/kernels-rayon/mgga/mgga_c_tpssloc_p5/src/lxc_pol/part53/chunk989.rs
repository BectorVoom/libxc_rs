//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 989/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk989(t1484: f64, t2047: f64, t22986: f64, t6646: f64, t829: f64, t22893: f64, t23164: f64, t33375: f64, t33383: f64, t6562: f64, t794: f64, t234: f64, t7823: f64) -> (f64, f64, f64, f64, f64) {
    let t121495 = t2047 * t1484;
    let t121498 = t22986 * t6646 * t121495 * t829;
    let t121501 = t23164 * t22893 * t33375;
    let t121504 = t6562 * t794 * t33383;
    let t121506 = t234 * t7823;
    (t121495, t121498, t121501, t121504, t121506)
}
