//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 912/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk912<F: Float>(t1262: F, t4621: F, t15239: F, t13475: F, t5310: F, t1071: F, t1851: F, t2630: F, t3515: F, t13495: F, t829: F) -> (F, F, F, F, F) {
    let t15240 = t4621 * t1262;
    let t15241 = t15239 * t15240;
    let t15244 = t5310 * t13475;
    let t15247 = t1851 * t1071;
    let t15248 = t15247 * t2630;
    let t15249 = t3515 * t15248;
    let t15252 = t5310 * t13495;
    let t15255 = t829 * t1262;
    (t15241, t15244, t15249, t15252, t15255)
}
