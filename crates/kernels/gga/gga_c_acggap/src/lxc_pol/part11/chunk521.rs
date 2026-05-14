//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 521/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk521<F: Float>(t1106: F, t1181: F, t423: F, t3361: F, t1111: F, t1165: F, t3189: F, t160: F, t413: F, t168: F, t1160: F, t1167: F, t1162: F, t3077: F, t1159: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3363 = t1181 * t423 * t1106;
    let t3364 = t3361 * t3363;
    let t3367 = t1165 * t3189 * t1111;
    let t3368 = t3361 * t3367;
    let t3370 = t160 * t413;
    let t3371 = t3370 * t168;
    let t3372 = t1160 * t3371;
    let t3373 = t3372 * t1167;
    let t3375 = t3077 * t1162;
    let t3376 = t3375 * t1167;
    let t3378 = t310 * t1159;
    (t3363, t3364, t3367, t3368, t3370, t3371, t3372, t3373, t3375, t3376, t3378)
}
