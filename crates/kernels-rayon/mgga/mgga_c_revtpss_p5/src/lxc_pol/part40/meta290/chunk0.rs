//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1042/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1042(t10521: f64, t231: f64, t268: f64, t2798: f64, t251: f64, t4503: f64, t786: f64, t2723: f64, t2453: f64, t2797: f64, t281: f64, t68: f64, t836: f64) -> (f64, f64, f64, f64, f64) {
    let t10523 = t268 * t10521 * t231;
    let t10524 = t2798 * t10523;
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10532 = t268 * t10521 * t2723;
    let t10533 = t10530 * t10532;
    let t10535 = t2453 * t2797;
    let t10538 = t281 * t68 * t836 * t231;
    (t10524, t10529, t10533, t10535, t10538)
}
