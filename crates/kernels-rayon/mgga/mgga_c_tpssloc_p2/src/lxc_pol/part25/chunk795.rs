//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 795/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk795(t2830: f64, t699: f64, t2833: f64, t241: f64, t2978: f64, t10216: f64, t9288: f64, t136: f64, t10277: f64, t2826: f64, t10195: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10300 = t699 * t2830;
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10305 = t10216 * t9288;
    let t10306 = t10304 * t10305;
    let t10307 = t136 * t10306;
    let t10309 = t10277 * t9288;
    let t10310 = t2826 * t10309;
    let t10311 = t136 * t10310;
    let t10313 = t2826 * t10195;
    let t10314 = t136 * t10313;
    let t10316 = t2770 * t9288;
    (t10300, t10302, t10305, t10307, t10309, t10311, t10314, t10316)
}
