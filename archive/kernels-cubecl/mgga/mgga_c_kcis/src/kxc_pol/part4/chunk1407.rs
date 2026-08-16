//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1407/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1407<F: Float>(t17377: F, t17380: F, t17384: F, t17386: F, t17389: F, t17392: F, t17394: F, t17398: F, t17400: F, t17403: F, t17405: F, t17407: F, t17410: F, t17413: F, t17415: F, t17418: F, t17421: F, t17423: F) -> F {
    let t18311 = -F::cast_from(0.10791666666666666667e0_f64) * t17377 + F::cast_from(0.375e0_f64) * t17380 + F::cast_from(0.27777777777777777777e-1_f64) * t17384 + F::cast_from(0.5e0_f64) * t17386 - F::cast_from(0.20234375e-1_f64) * t17389 + F::cast_from(0.26979166666666666666e-1_f64) * t17392 - F::cast_from(0.13489583333333333333e-1_f64) * t17394 + F::cast_from(0.1875e0_f64) * t17398 + F::cast_from(0.26979166666666666666e-1_f64) * t17400 - F::cast_from(0.20833333333333333333e-1_f64) * t17403 + F::cast_from(0.625e-1_f64) * t17405 - F::cast_from(0.125e0_f64) * t17407 - F::cast_from(0.625e-1_f64) * t17410 - F::cast_from(0.125e0_f64) * t17413 + F::cast_from(0.125e0_f64) * t17415 - F::cast_from(0.4046875e-1_f64) * t17418 - F::cast_from(0.5e0_f64) * t17421 - F::cast_from(0.20833333333333333333e-1_f64) * t17423;
    t18311
}
