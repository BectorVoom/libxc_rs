//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 370/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk370<F: Float>(t1247: F, t129: F, t1240: F, t3097: F, t3091: F, t464: F, t866: F, t3095: F, t3099: F, t869: F, t871: F, t1232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3101 = t1247 * t129;
    let t3102 = t3097 * t1240;
    let t3103 = t3102 * pi;
    let t3104 = t3101 * t3103;
    let t3106 = t464 * t3091;
    let t3107 = t3106 * t866;
    let t3108 = t3107 / F::cast_from(256.0_f64);
    let t3109 = t3095 - F::cast_from(9.0_f64) / F::cast_from(8192.0_f64) * t3099 + F::cast_from(3.0_f64) / F::cast_from(8192.0_f64) * t3104 - t3108;
    let t3111 = t869 * t871;
    let t3113 = F::cast_from(1.0_f64) / t1232;
    (t3101, t3102, t3103, t3104, t3106, t3107, t3108, t3109, t3111, t3113)
}
