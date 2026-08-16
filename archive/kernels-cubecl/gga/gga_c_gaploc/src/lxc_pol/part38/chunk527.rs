//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 527/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk527<F: Float>(t10135: F, t2268: F, t3355: F, t6305: F, t7995: F, t874: F, t2343: F, t2293: F, t2787: F, t10115: F, t10118: F, t10119: F, t10124: F, t10127: F, t10131: F, t10134: F, t1063: F, t9072: F, t9077: F, t9085: F) -> (F, F, F) {
    let t10137 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t10135;
    let t10139 = F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t3355;
    let t10140 = t7995 * t874;
    let t10141 = t2343 * t10140;
    let t10143 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10141;
    let t10144 = t2787 * t2293;
    let t10145 = t2343 * t10144;
    let t10147 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t10145;
    let t10148 = -t9072 + t9077 + t9085 + t10115 + t10118 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t10119 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t10124 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t10127 - t10131 - t10134 - t10137 + t10139 + t10143 + t10147;
    (t10140, t10144, t10148)
}
