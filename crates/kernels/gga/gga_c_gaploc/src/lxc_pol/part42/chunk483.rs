//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 483/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk483<F: Float>(t10040: F, t7390: F, t2440: F, t988: F, t2268: F, t2756: F, t894: F, t3347: F, t6305: F, t7930: F, t888: F, t2349: F, t2765: F, t3355: F, t7995: F, t874: F) -> (F, F, F, F, F, F, F, F) {
    let t10042 = 0.29792074959875355558e-1 * t7390 * t10040;
    let t10113 = t2440 * t988;
    let t10115 = 0.28455006635676149599e-1 * t2268 * t10113;
    let t10116 = t894 * t2756;
    let t10118 = 0.28455006635676149599e-1 * t2268 * t10116;
    let t10131 = 0.85365019907028448797e-1 * t6305 * t3347;
    let t10132 = t7930 * t888;
    let t10134 = 0.85365019907028448797e-1 * t2268 * t10132;
    let t10135 = t2765 * t2349;
    let t10137 = 0.85365019907028448797e-1 * t2268 * t10135;
    let t10139 = 0.56910013271352299198e-1 * t6305 * t3355;
    let t10140 = t7995 * t874;
    (t10042, t10115, t10118, t10131, t10134, t10137, t10139, t10140)
}
