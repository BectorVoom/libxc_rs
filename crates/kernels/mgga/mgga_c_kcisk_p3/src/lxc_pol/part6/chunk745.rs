//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 745/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk745<F: Float>(t116: F, t5821: F, t114: F, t923: F, t3052: F, t927: F, t123: F, t6: F, t120: F, t20: F, t3050: F, t3058: F, t397: F) -> (F, F, F, F) {
    let t15244 = t116 * t5821;
    let t15245 = t114 * t15244;
    let t15250 = t923 * t923;
    let t15251 = F::cast_from(1.0_f64) / t15250;
    let t15253 = t3052 * t927;
    let t15255 = t123 * t6 * t15251 * t15253;
    let t15258 = t120 * t20;
    let t15259 = t114 * t15258;
    let t15260 = t3050 * t927;
    let t15262 = t397 * t15260 * t3058;
    (t15245, t15255, t15259, t15262)
}
