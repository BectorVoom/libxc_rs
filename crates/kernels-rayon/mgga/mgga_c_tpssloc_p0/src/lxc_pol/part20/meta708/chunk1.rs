//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2711/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711(t1851: f64, t3946: f64, t1858: f64, t3931: f64, t1395: f64, t5381: f64, t1404: f64, t5363: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t16507: f64, t16546: f64, t1852: f64, t3: f64, t39022: f64, t39024: f64, t39026: f64, t39028: f64, t3932: f64, t45584: f64, t45588: f64, t5364: f64, t55317: f64, t55364: f64, t580: f64) -> f64 {
    let t55368 = t1851 * t3946;
    let t55374 = t3931 * t1858;
    let t55376 = t1395 * t5381;
    let t55378 = t5363 * t1404;
    let tv4rho41 = t3 * t55317 * t580 + t12513 * t1858 + t12537 * t1852 + 3.0_f64 * t1396 * t16546 + t1398 * t55364 + 3.0_f64 * t1404 * t16507 + 3.0_f64 * t3932 * t5381 + 3.0_f64 * t3946 * t5364 + t39022 + 3.0_f64 * t39024 + 3.0_f64 * t39026 + t39028 + 3.0_f64 * t45584 + 3.0_f64 * t45588 + 3.0_f64 * t55368 + 3.0_f64 * t55374 + 6.0_f64 * t55376 + 6.0_f64 * t55378;
    tv4rho41
}
