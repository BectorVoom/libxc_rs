//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2711/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711<F: Float>(t1851: F, t3946: F, t1858: F, t3931: F, t1395: F, t5381: F, t1404: F, t5363: F, t12513: F, t12537: F, t1396: F, t1398: F, t16507: F, t16546: F, t1852: F, t3: F, t39022: F, t39024: F, t39026: F, t39028: F, t3932: F, t45584: F, t45588: F, t5364: F, t55317: F, t55364: F, t580: F) -> F {
    let t55368 = t1851 * t3946;
    let t55374 = t3931 * t1858;
    let t55376 = t1395 * t5381;
    let t55378 = t5363 * t1404;
    let tv4rho41 = t3 * t55317 * t580 + t12513 * t1858 + t12537 * t1852 + F::cast_from(3.0_f64) * t1396 * t16546 + t1398 * t55364 + F::cast_from(3.0_f64) * t1404 * t16507 + F::cast_from(3.0_f64) * t3932 * t5381 + F::cast_from(3.0_f64) * t3946 * t5364 + t39022 + F::cast_from(3.0_f64) * t39024 + F::cast_from(3.0_f64) * t39026 + t39028 + F::cast_from(3.0_f64) * t45584 + F::cast_from(3.0_f64) * t45588 + F::cast_from(3.0_f64) * t55368 + F::cast_from(3.0_f64) * t55374 + F::cast_from(6.0_f64) * t55376 + F::cast_from(6.0_f64) * t55378;
    tv4rho41
}
