//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 747/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk747<F: Float>(t95: F, t257: F, t260: F, t277: F, t255: F, t6311: F, t6314: F, t254: F, t510: F, t116: F, t122: F, t3436: F, t57: F) -> (F, F, F, F, F) {
    let t6317 = t95 * t95;
    let t6319 = F::new(1.0) / t257 / t6317;
    let t6321 = t6319 * t260 * t277;
    let t6322 = t6311 * t6314 * t255 * t6321;
    let t6324 = F::cast_from(0.41530324072742201648e-1_f64) * t254 * t6322;
    let t6325 = t510 * t510;
    let t6326 = F::new(1.0) / t6325;
    let t6327 = t116 * t6326;
    let t6329 = t122 * t3436 * t57;
    (t6319, t6321, t6324, t6327, t6329)
}
