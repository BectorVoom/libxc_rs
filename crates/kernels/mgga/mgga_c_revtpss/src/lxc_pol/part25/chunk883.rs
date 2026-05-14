//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 883/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk883<F: Float>(t1079: F, t11183: F, t1071: F, t3057: F, t3259: F, t994: F, t342: F, t992: F, t338: F, t378: F, t3059: F, t999: F, t996: F, t3325: F, t3043: F, t3042: F, t993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11184 = t1079 * t11183;
    let t11187 = t3057 * t1071;
    let t11190 = t994 * t3259;
    let t11195 = t342 * t3259;
    let t11198 = t992 * t992;
    let t11199 = 1.0 / t11198;
    let t11200 = t338 * t11199;
    let t11201 = t11200 * t378;
    let t11202 = t3059 * t999;
    let t11203 = t996 * t11202;
    let t11206 = t999 * t3325;
    let t11207 = t1079 * t11206;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    (t11184, t11187, t11190, t11195, t11198, t11199, t11200, t11201, t11202, t11203, t11207, t11210, t11213)
}
