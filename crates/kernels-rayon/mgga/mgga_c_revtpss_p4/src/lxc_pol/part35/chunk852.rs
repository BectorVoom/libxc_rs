//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 852/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk852(t140: f64, t6658: f64, t1222: f64, t6662: f64, t369: f64, t6593: f64, t475: f64, t467: f64, t1256: f64, t6602: f64, t6595: f64, t6598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21251 = t140 * t6658;
    let t21252 = t1222 * t21251;
    let t21254 = t140 * t6662;
    let t21255 = t1222 * t21254;
    let t21270 = t6593 * t369;
    let t21271 = t475 * t21270;
    let t21272 = t467 * t21271;
    let t21283 = t6602 * t1256;
    let t21285 = t6595 * t1256;
    let t21287 = t6598 * t1256;
    (t21252, t21255, t21272, t21283, t21285, t21287)
}
