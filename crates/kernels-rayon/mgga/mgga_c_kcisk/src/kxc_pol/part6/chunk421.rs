//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 421/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk421(t60: f64, t116: f64, t3042: f64, t114: f64, t126: f64, t923: f64, t6: f64, t927: f64, t123: f64, t2925: f64, t925: f64, t121: f64, t129: f64, t3033: f64, t3036: f64, t913: f64, t920: f64, t929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t124 = 0.0_f64 < t60;
    let t3043 = t116 * t3042;
    let t3044 = t114 * t3043;
    let t3050 = 1.0_f64 / t923 / t126;
    let t3051 = t6 * t3050;
    let t3052 = t927 * t927;
    let t3054 = t123 * t3051 * t3052;
    let t3058 = piecewise3(t124, t2925, -t2925);
    let t3060 = t123 * t925 * t3058;
    let t3063 = 0.53972366148531951642e-1_f64 * t3033 * t129 - 0.251871042026482441e0_f64 * t3036 * t129 - 0.10794473229706390328e0_f64 * t913 * t929 + 0.41978507004413740167e0_f64 * t3044 * t129 + 0.251871042026482441e0_f64 * t920 * t929 + 0.10794473229706390328e0_f64 * t121 * t3054 - 0.53972366148531951642e-1_f64 * t121 * t3060;
    (t3043, t3044, t3050, t3052, t3054, t3058, t3060, t3063)
}
