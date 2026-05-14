//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 823/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk823<F: Float>(t31022: F, t130: F, t1964: F, t2037: F, t377: F, t7684: F, t409: F, t1: F, t2065: F, t2066: F, t1160: F) -> (F, F, F, F, F, F, F) {
    let t31023 = 0.60023625365297631762e-2 * t31022;
    let t31035 = t130 * t1964;
    let t31036 = t31035 * t2037;
    let t31037 = 311.0 / 864.0 * t31036;
    let t31038 = t377 * t7684;
    let t31039 = t31038 * t409;
    let t31056 = t2065 * t2066 * t1;
    let t31057 = t1160 * t31056;
    (t31023, t31035, t31037, t31038, t31039, t31056, t31057)
}
