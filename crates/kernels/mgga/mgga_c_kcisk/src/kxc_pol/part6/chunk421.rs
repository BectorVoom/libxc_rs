//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 421/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk421<F: Float>(t60: F, t116: F, t3042: F, t114: F, t126: F, t923: F, t6: F, t927: F, t123: F, t2925: F, t925: F, t121: F, t129: F, t3033: F, t3036: F, t913: F, t920: F, t929: F) -> (F, F, F, F, F, F, F, F) {
    let t124 = F::new(0.0) < t60;
    let t3043 = t116 * t3042;
    let t3044 = t114 * t3043;
    let t3050 = F::new(1.0) / t923 / t126;
    let t3051 = t6 * t3050;
    let t3052 = t927 * t927;
    let t3054 = t123 * t3051 * t3052;
    let t3058 = piecewise3::<f64>(t124, t2925, -t2925);
    let t3060 = t123 * t925 * t3058;
    let t3063 = F::new(0.53972366148531951642e-1) * t3033 * t129 - F::new(0.251871042026482441e0) * t3036 * t129 - F::new(0.10794473229706390328e0) * t913 * t929 + F::new(0.41978507004413740167e0) * t3044 * t129 + F::new(0.251871042026482441e0) * t920 * t929 + F::new(0.10794473229706390328e0) * t121 * t3054 - F::new(0.53972366148531951642e-1) * t121 * t3060;
    (t3043, t3044, t3050, t3052, t3054, t3058, t3060, t3063)
}
