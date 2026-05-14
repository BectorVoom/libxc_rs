//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1114/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1114<F: Float>(t10043: F, t2394: F, t3265: F, t3898: F, t2381: F, t3026: F, t394: F, t1249: F, t3192: F, t3214: F, t3860: F, t927: F, t2886: F, t980: F, t2382: F, t3913: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10047 = t2394 * t10043;
    let t10050 = t3265 * t3898;
    let t10051 = t2381 * t10050;
    let t10054 = t394 * t3026;
    let t10055 = t1249 * t10054;
    let t10056 = t2381 * t10055;
    let t10059 = t3214 * t3192;
    let t10061 = t3860 * t927;
    let t10063 = t980 * t2886;
    let t10066 = t3913 * t2382;
    (t10047, t10050, t10051, t10054, t10055, t10056, t10059, t10061, t10063, t10066)
}
