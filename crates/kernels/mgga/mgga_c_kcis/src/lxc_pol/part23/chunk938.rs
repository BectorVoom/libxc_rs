//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 938/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk938<F: Float>(t15860: F, t5909: F, t4260: F, t12265: F, t4291: F, t6012: F, t17331: F, t17335: F, t17337: F, t17339: F, t17342: F, t17344: F, t17347: F, t17350: F, t17353: F, t17355: F, t17358: F, t17360: F, t17362: F, t17364: F, t17366: F, t17368: F) -> (F, F, F, F) {
    let t17370 = t5909 * t15860;
    let t17371 = t4260 * t17370;
    let t17373 = t12265 * t4291;
    let t17374 = t17373 * t6012;
    let t17376 = -t17331 / F::new(256.0) - t17335 / F::new(48.0) + t17337 / F::new(24.0) - F::new(2.0) / F::new(9.0) * t17339 - t17342 / F::new(576.0) - t17344 / F::new(8.0) - t17347 / F::new(36.0) + t17350 / F::new(576.0) + t17353 / F::new(24.0) - t17355 / F::new(16.0) + t17358 / F::new(4.0) - t17360 / F::new(16.0) + t17362 / F::new(48.0) - t17364 / F::new(12.0) + t17366 / F::new(96.0) - t17368 / F::new(576.0) + t17371 / F::new(72.0) - t17374 / F::new(64.0);
    (t17370, t17371, t17374, t17376)
}
