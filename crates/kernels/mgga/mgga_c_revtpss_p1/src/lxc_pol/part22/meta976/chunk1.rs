//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3285/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285<F: Float>(t62282: F, t1522: F, t49880: F, t50878: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t61310: F, t61311: F, t61313: F, t61316: F, t61317: F, t62269: F, t62270: F, t62273: F, t62275: F, t62277: F, t62279: F) -> (F, F, F, F) {
    let t62283 = F::new(48.0) * t62282;
    let t62285 = F::new(8.0) * t49880 * t1522;
    let t62286 = F::new(24.0) * t50878;
    let t62287 = t61310 + t61311 + t61313 + t61316 - t61317 + t40067 - t40072 + t62269 + t40167 - t40171 - t62270 - t40184 + t62273 + t62275 + t62277 + t62279 + t62283 + t62285 + t62286;
    (t62283, t62285, t62286, t62287)
}
