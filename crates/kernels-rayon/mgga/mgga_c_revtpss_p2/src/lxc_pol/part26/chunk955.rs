//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 955/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk955(t12274: f64, t141: f64, t10326: f64, t1121: f64, t1145: f64, t3362: f64, t606: f64, t2258: f64, t3417: f64, t3367: f64, t3360: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12275 = t141 * t12274;
    let t12277 = t1121 * t10326;
    let t12278 = t1145 * t12277;
    let t12279 = t141 * t12278;
    let t12281 = t3362 * t606;
    let t12282 = t12281 * t2258;
    let t12283 = t3417 * t12282;
    let t12284 = t141 * t12283;
    let t12286 = t3367 * t606;
    let t12287 = t12286 * t2258;
    let t12288 = t1145 * t12287;
    let t12289 = t141 * t12288;
    let t12291 = t3360 * t12282;
    let t12292 = t128 * t12291;
    (t12275, t12277, t12279, t12282, t12284, t12287, t12289, t12292)
}
