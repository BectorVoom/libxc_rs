//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 187/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk187<F: Float>(t169: F, t935: F, t299: F, t706: F, t268: F, t78: F, t278: F, t481: F) -> (F, F, F, F) {
    let t936 = t935 * t169;
    let t937 = t936 * t299;
    let t938 = t706 * t937;
    let t941 = t78 * t268;
    let t943 = t481 * t941 * t278;
    (t937, t938, t941, t943)
}
