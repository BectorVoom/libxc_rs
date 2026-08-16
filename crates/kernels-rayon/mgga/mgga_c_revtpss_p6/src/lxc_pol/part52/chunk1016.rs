//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1016/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1016(t31772: f64, t4364: f64, t886: f64, t31767: f64, t2769: f64, t8648: f64, t8476: f64, t9645: f64) -> (f64, f64, f64, f64) {
    let t31774 = t4364 * t31772 * t886;
    let t31775 = t31767 * t31774;
    let t31798 = t8648 * t2769;
    let t31805 = t8476 * t9645;
    (t31774, t31775, t31798, t31805)
}
