//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1000/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1000<F: Float>(t17209: F, t17264: F, t17290: F, t17320: F, t17321: F, t17327: F, t17330: F, t17335: F, t17339: F, t17342: F, t17346: F, t17351: F, t17730: F, t1778: F, t5013: F, t5022: F, t7219: F) -> (F,) {
    let t17733 = t17209 + t17264 + t17320 - 0.1439263097294185377e0 * t5013 * t17321 + 0.35981577432354634426e-1 * t17290 * t1778 - 0.39979530480394038251e-2 * t17327 + t17330 + 0.95950873152945691806e-1 * t7219 * t5022 - t17335 + 0.71963154864709268852e-1 * t5013 * t17339 - 0.35981577432354634426e-1 * t5013 * t17342 + 0.1439263097294185377e0 * t5013 * t17346 - t17351 + t17730;
    (t17733,)
}
