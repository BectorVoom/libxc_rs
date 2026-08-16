//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1086/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1086(t1087: f64, t11671: f64, t3090: f64, t3278: f64, t3133: f64, t73: f64, t2258: f64, t3094: f64, t3182: f64, t828: f64, t2852: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11672 = t1087 * t11671;
    let t11675 = t3278 * t3090;
    let t11678 = t3133 * t73;
    let t11696 = t3094 * t2258;
    let t11703 = t828 * t3182;
    let t11704 = t357 * t2852;
    (t11672, t11675, t11678, t11696, t11703, t11704)
}
