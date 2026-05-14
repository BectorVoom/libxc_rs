//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 382/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk382<F: Float>(t2610: F, t935: F, t2365: F, t2033: F, t3251: F, t531: F, t3255: F, t3209: F, t808: F, t568: F, t123: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t3280 = t2610 * t935;
    let t3281 = t2365 * t3280;
    let t3283 = 0.29792074959875355558e-1 * t2033 * t3281;
    let t3284 = t531 * t3251;
    let t3287 = t531 * t3255;
    let t3290 = t808 * t3209;
    let t3291 = t568 * t3290;
    let t3294 = t935 * t123;
    let t3295 = t3294 * t883;
    (t3280, t3281, t3283, t3284, t3287, t3290, t3291, t3295)
}
