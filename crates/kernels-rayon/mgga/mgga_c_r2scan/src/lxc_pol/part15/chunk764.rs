//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 764/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk764(t95: f64, t257: f64, t260: f64, t277: f64, t255: f64, t6311: f64, t6314: f64, t254: f64, t510: f64, t116: f64, t122: f64, t3436: f64, t57: f64) -> (f64, f64, f64, f64, f64) {
    let t6317 = t95 * t95;
    let t6319 = 1.0_f64 / t257 / t6317;
    let t6321 = t6319 * t260 * t277;
    let t6322 = t6311 * t6314 * t255 * t6321;
    let t6324 = 0.41530324072742201648e-1_f64 * t254 * t6322;
    let t6325 = t510 * t510;
    let t6326 = 1.0_f64 / t6325;
    let t6327 = t116 * t6326;
    let t6329 = t122 * t3436 * t57;
    (t6319, t6321, t6324, t6327, t6329)
}
