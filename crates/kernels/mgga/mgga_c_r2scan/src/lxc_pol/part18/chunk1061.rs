//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1061/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1061<F: Float>(t10810: F, t1592: F, t9380: F, t3190: F, t3319: F, t3320: F, t5103: F, t37947: F, t37951: F, t41600: F, t43348: F, t43351: F, t43356: F, t43359: F, t43362: F, t43365: F) -> (F,) {
    let t43368 = t1592 * t10810 * t9380;
    let t43372 = t5103 * t3319 * t3320 * t3190;
    let t43374 = 0.52396431978519890152e-1 * t43348 + 0.10401866088065122276e1 * t43351 + 0.15573871527278325618e-1 * t37947 + 0.46721614581834976854e-1 * t37951 - 0.47609969197673950971e-2 * t43356 + 0.13099107994629972538e-1 * t43359 - 0.28565981518604370583e-1 * t43362 + 0.71414953796510926457e-2 * t43365 - 0.69345773920434148507e0 * t43368 + 0.46574606203128791245e-1 * t43372 + t41600;
    (t43374,)
}
