//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 354/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk354<F: Float>(t3106: F, t866: F, t3095: F, t3099: F, t3104: F, t869: F, t871: F, t1232: F) -> (F, F, F, F, F) {
    let t3107 = t3106 * t866;
    let t3108 = t3107 / 256.0;
    let t3109 = t3095 - 9.0 / 8192.0 * t3099 + 3.0 / 8192.0 * t3104 - t3108;
    let t3111 = t869 * t871;
    let t3113 = 1.0 / t1232;
    (t3107, t3108, t3109, t3111, t3113)
}
