//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 604/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk604(t10135: f64, t2268: f64, t3355: f64, t6305: f64, t7995: f64, t874: f64, t2343: f64, t2293: f64, t2787: f64, t10115: f64, t10118: f64, t10119: f64, t10124: f64, t10127: f64, t10131: f64, t10134: f64, t1063: f64, t9072: f64, t9077: f64, t9085: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10137 = 0.85365019907028448797e-1_f64 * t2268 * t10135;
    let t10139 = 0.56910013271352299198e-1_f64 * t6305 * t3355;
    let t10140 = t7995 * t874;
    let t10141 = t2343 * t10140;
    let t10143 = 0.56910013271352299198e-1_f64 * t2268 * t10141;
    let t10144 = t2787 * t2293;
    let t10145 = t2343 * t10144;
    let t10147 = 0.56910013271352299198e-1_f64 * t2268 * t10145;
    let t10148 = -t9072 + t9077 + t9085 + t10115 + t10118 - 0.28455006635676149599e-1_f64 * t1063 * t10119 + 0.28455006635676149599e-1_f64 * t1063 * t10124 + 0.28455006635676149599e-1_f64 * t2268 * t10127 - t10131 - t10134 - t10137 + t10139 + t10143 + t10147;
    (t10137, t10139, t10140, t10143, t10144, t10147, t10148)
}
