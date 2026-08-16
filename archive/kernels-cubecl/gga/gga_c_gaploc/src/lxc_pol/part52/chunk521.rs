//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 521/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk521<F: Float>(t10116: F, t2268: F, t3347: F, t6305: F, t7930: F, t888: F, t2349: F, t2765: F, t3355: F, t7995: F, t874: F, t2343: F) -> (F, F, F, F, F, F, F) {
    let t10118 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t10116;
    let t10131 = F::cast_from(0.85365019907028448797e-1_f64) * t6305 * t3347;
    let t10132 = t7930 * t888;
    let t10134 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t10132;
    let t10135 = t2765 * t2349;
    let t10137 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t10135;
    let t10139 = F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t3355;
    let t10140 = t7995 * t874;
    let t10141 = t2343 * t10140;
    (t10118, t10131, t10134, t10137, t10139, t10140, t10141)
}
