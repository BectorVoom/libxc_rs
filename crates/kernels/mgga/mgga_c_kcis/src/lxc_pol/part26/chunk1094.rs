//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1094/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1094<F: Float>(t1386: F, t1938: F, t1380: F, t28372: F, t1497: F, t15910: F, t7923: F, t1464: F, t27335: F, t27337: F, t27346: F, t27349: F, t27362: F, t27369: F, t27377: F, t28329: F, t28333: F, t28336: F, t28340: F, t28344: F, t28348: F, t28353: F, t28358: F, t28362: F, t28369: F, t7908: F, t7911: F) -> (F, F, F, F, F, F, F) {
    let t28373 = t1386 * t1938;
    let t28374 = t28373 * t1380;
    let t28375 = t28372 * t28374;
    let t28382 = t15910 * t1497;
    let t28383 = t7923 * t28382;
    let t28384 = t1464 * t28383;
    let t28386 = F::cast_from(0.16581944444444444444e-2_f64) * t28329 - F::cast_from(0.33163888888888888888e-2_f64) * t28333 - F::cast_from(0.7722800925925925926e-4_f64) * t28336 - F::cast_from(0.16581944444444444444e-2_f64) * t28340 - F::cast_from(0.92754700520833333333e-4_f64) * t27369 * t28344 - F::cast_from(0.69505208333333333333e-3_f64) * t7908 * t28348 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t28353 - F::cast_from(0.44218518518518518517e-2_f64) * t28358 - F::cast_from(0.16581944444444444444e-2_f64) * t28362 - F::cast_from(0.16581944444444444444e-2_f64) * t27335 + F::cast_from(0.11054629629629629629e-2_f64) * t27337 + F::cast_from(0.23168402777777777778e-3_f64) * t27346 + F::cast_from(0.23168402777777777778e-3_f64) * t27349 - F::cast_from(0.23168402777777777778e-3_f64) * t28369 * t7911 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t28375 - F::cast_from(0.69505208333333333333e-3_f64) * t7908 * t28344 + F::cast_from(0.11054629629629629629e-2_f64) * t27362 - F::cast_from(0.7722800925925925926e-4_f64) * t27377 - F::cast_from(0.55273148148148148147e-3_f64) * t28384;
    (t28373, t28374, t28375, t28382, t28383, t28384, t28386)
}
