//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1204/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1204<F: Float>(t37947: F, t37951: F, t41600: F, t43348: F, t43351: F, t43356: F, t43359: F, t43362: F, t43365: F, t43368: F, t43372: F, t22790: F, t30057: F, t3332: F) -> (F, F) {
    let t43374 = F::cast_from(0.52396431978519890152e-1_f64) * t43348 + F::cast_from(0.10401866088065122276e1_f64) * t43351 + F::cast_from(0.15573871527278325618e-1_f64) * t37947 + F::cast_from(0.46721614581834976854e-1_f64) * t37951 - F::cast_from(0.47609969197673950971e-2_f64) * t43356 + F::cast_from(0.13099107994629972538e-1_f64) * t43359 - F::cast_from(0.28565981518604370583e-1_f64) * t43362 + F::cast_from(0.71414953796510926457e-2_f64) * t43365 - F::cast_from(0.69345773920434148507e0_f64) * t43368 + F::cast_from(0.46574606203128791245e-1_f64) * t43372 + t41600;
    let t43376 = t22790 * t3332 * t30057;
    (t43374, t43376)
}
