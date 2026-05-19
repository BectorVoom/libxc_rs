//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 216/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk216<F: Float>(t60: F, t116: F, t918: F, t114: F, t126: F, t6: F, t852: F, t123: F, t121: F, t129: F, t913: F, t132: F, t119: F, t177: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t124 = F::new(0.0) < t60;
    let t919 = t116 * t918;
    let t920 = t114 * t919;
    let t923 = t126 * t126;
    let t924 = F::new(1.0) / t923;
    let t925 = t6 * t924;
    let t927 = piecewise3::<F>(t124, t852, -t852);
    let t929 = t123 * t925 * t927;
    let t932 = F::cast_from(0.53972366148531951642e-1_f64) * t913 * t129 - F::cast_from(0.1259355210132412205e0_f64) * t920 * t129 - F::cast_from(0.53972366148531951642e-1_f64) * t121 * t929;
    let t933 = F::new(1.0) / t132;
    let t934 = t932 * t933;
    let t937 = t119 * t177;
    (t919, t920, t923, t925, t927, t929, t932, t933, t934, t937)
}
