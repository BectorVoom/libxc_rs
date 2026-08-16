//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2985/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985<F: Float>(t10390: F, t10403: F, t10480: F, t10883: F, t13985: F, t17670: F, t17677: F, t17705: F, t17712: F, t17980: F, t2776: F, t3041: F, t3071: F, t3121: F, t3132: F, t42347: F, t42354: F, t42358: F, t42496: F, t4582: F, t49940: F, t49945: F, t49957: F, t49959: F, t49964: F, t49966: F, t5873: F, t5909: F) -> F {
    let t62398 = t10480 * t4582 * t17712 * t13985 / F::cast_from(512.0_f64) + t42354 * t17980 / F::cast_from(1536.0_f64) + t10883 * t4582 * t17670 * t3121 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t42347 * t4582 * t17670 * t3132 - t42358 * t4582 * t17670 * t3041 / F::cast_from(3072.0_f64) + t49940 / F::cast_from(1152.0_f64) - t49945 / F::cast_from(1728.0_f64) - t10403 * t3071 * t5873 * t2776 / F::cast_from(1152.0_f64) + t49957 / F::cast_from(1152.0_f64) - t49959 / F::cast_from(2304.0_f64) + t49964 / F::cast_from(1152.0_f64) + t49966 / F::cast_from(1728.0_f64) + t10390 * t17677 / F::cast_from(1152.0_f64) - t42496 * t5909 / F::cast_from(216.0_f64) + t10390 * t17705 / F::cast_from(1152.0_f64);
    t62398
}
