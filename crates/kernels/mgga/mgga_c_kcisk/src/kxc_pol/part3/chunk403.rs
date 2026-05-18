//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 403/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk403<F: Float>(t852: F, t855: F, t135: F, t854: F, t60: F, t932: F, t132: F, t68: F, t69: F, t142: F, t862: F, t85: F, t861: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2927 = t852 * t855;
    let t2931 = F::new(1.0) / t854 / t135;
    let t2932 = t60 * t2931;
    let t2933 = t932 * t932;
    let t2934 = t132 * t132;
    let t2935 = F::new(1.0) / t2934;
    let t2936 = t2933 * t2935;
    let t2942 = t68 * t69;
    let t2943 = t142 * t862;
    let t2947 = t861 * t85;
    (t2927, t2931, t2932, t2933, t2934, t2935, t2936, t2942, t2943, t2947)
}
