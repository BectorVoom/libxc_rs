//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 605/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk605(t225: f64, t2760: f64, t213: f64, t860: f64, t256: f64, t866: f64, t886: f64, t2435: f64, t871: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2761 = t2760 * t225;
    let t2765 = t213 * t860;
    let t2769 = 1.0_f64 / t866 / t256;
    let t2770 = t225 * t2769;
    let t2771 = t886 * t886;
    let t2772 = t2770 * t2771;
    let t2776 = 0.73171657588172351096e-2_f64 * t2435 * t871;
    let t2777 = t785 * t225;
    (t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777)
}
