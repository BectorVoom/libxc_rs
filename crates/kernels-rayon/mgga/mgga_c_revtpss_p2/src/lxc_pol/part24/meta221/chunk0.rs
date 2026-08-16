//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 972/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk972(t12051: f64, t357: f64, t11239: f64, t3143: f64, t342: f64, t3154: f64, t4980: f64, t994: f64, t4995: f64, t3057: f64, t3286: f64, t11627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12052 = t12051 * t357;
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12122 = t994 * t4980;
    let t12127 = t994 * t4995;
    let t12149 = t3057 * t3286;
    let t12166 = t11239 * t11627;
    (t12052, t12077, t12078, t12079, t12122, t12127, t12149, t12166)
}
