//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 532/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk532<F: Float>(t10132: F, t2268: F, t2349: F, t2765: F, t3355: F, t6305: F, t7995: F, t874: F, t2343: F, t2293: F, t2787: F, t3327: F, t6313: F, t2317: F, t2761: F, t6525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10134 = 0.85365019907028448797e-1 * t2268 * t10132;
    let t10135 = t2765 * t2349;
    let t10137 = 0.85365019907028448797e-1 * t2268 * t10135;
    let t10139 = 0.56910013271352299198e-1 * t6305 * t3355;
    let t10140 = t7995 * t874;
    let t10141 = t2343 * t10140;
    let t10143 = 0.56910013271352299198e-1 * t2268 * t10141;
    let t10144 = t2787 * t2293;
    let t10145 = t2343 * t10144;
    let t10147 = 0.56910013271352299198e-1 * t2268 * t10145;
    let t10150 = 0.37940008847568199465e-1 * t6313 * t3327;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    (t10134, t10137, t10139, t10140, t10143, t10144, t10147, t10150, t10161)
}
