//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1063/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1063<F: Float>(t4265: F, t8995: F, t140: F, t299: F, t9010: F, t23271: F, t673: F, t18076: F, t22392: F, t22396: F, t7370: F, t22387: F, t1797: F, t2505: F, t6764: F, t4594: F) -> (F, F, F, F, F, F, F, F) {
    let t24320 = t4265 * t8995;
    let t24324 = t140 * t299 * t9010;
    let t24326 = t673 * t23271;
    let t24332 = t18076 * t22392;
    let t24335 = t7370 * t22396;
    let t24338 = t7370 * t22387;
    let t24341 = t1797 * t2505;
    let t24342 = t24341 * t6764;
    let t24345 = t4594 * t2505;
    (t24320, t24324, t24326, t24332, t24335, t24338, t24342, t24345)
}
