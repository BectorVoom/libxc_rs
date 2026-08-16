//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1127/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1127(t33620: f64, t644: f64, t8621: f64, t7714: f64, t84: f64, t640: f64, t33624: f64, t6972: f64, t1497: f64, t36: f64, t606: f64, t1936: f64, t97622: f64) -> (f64, f64, f64, f64, f64) {
    let t125319 = t8621 * t33620 * t644;
    let t125322 = t84 * t7714;
    let t125324 = t8621 * t125322 * t640;
    let t125332 = t8621 * t33624 * t6972;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125355 = t97622 * t1936;
    (t125319, t125324, t125332, t125336, t125355)
}
