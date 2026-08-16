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
    let t17376 = -t17331 / F::cast_from(256.0_f64) - t17335 / F::cast_from(48.0_f64) + t17337 / F::cast_from(24.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17339 - t17342 / F::cast_from(576.0_f64) - t17344 / F::cast_from(8.0_f64) - t17347 / F::cast_from(36.0_f64) + t17350 / F::cast_from(576.0_f64) + t17353 / F::cast_from(24.0_f64) - t17355 / F::cast_from(16.0_f64) + t17358 / F::cast_from(4.0_f64) - t17360 / F::cast_from(16.0_f64) + t17362 / F::cast_from(48.0_f64) - t17364 / F::cast_from(12.0_f64) + t17366 / F::cast_from(96.0_f64) - t17368 / F::cast_from(576.0_f64) + t17371 / F::cast_from(72.0_f64) - t17374 / F::cast_from(64.0_f64);
    (t17370, t17371, t17374, t17376)
}
