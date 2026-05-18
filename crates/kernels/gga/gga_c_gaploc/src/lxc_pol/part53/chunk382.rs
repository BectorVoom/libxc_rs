//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 382/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk382<F: Float>(t2366: F, t874: F, t2365: F, t1429: F, t3133: F, t531: F, t3137: F, t3085: F, t569: F, t568: F, t123: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t3162 = t2366 * t874;
    let t3163 = t2365 * t3162;
    let t3165 = F::new(0.29792074959875355558e-1) * t1429 * t3163;
    let t3166 = t531 * t3133;
    let t3169 = t531 * t3137;
    let t3172 = t569 * t3085;
    let t3173 = t568 * t3172;
    let t3176 = t874 * t123;
    let t3177 = t3176 * t883;
    (t3162, t3163, t3165, t3166, t3169, t3172, t3173, t3177)
}
