//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 527/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk527<F: Float>(t160: F, t413: F, t168: F, t1160: F, t1167: F, t1162: F, t3077: F, t1159: F, t310: F) -> (F, F, F, F, F, F, F) {
    let t3370 = t160 * t413;
    let t3371 = t3370 * t168;
    let t3372 = t1160 * t3371;
    let t3373 = t3372 * t1167;
    let t3375 = t3077 * t1162;
    let t3376 = t3375 * t1167;
    let t3378 = t310 * t1159;
    (t3370, t3371, t3372, t3373, t3375, t3376, t3378)
}
