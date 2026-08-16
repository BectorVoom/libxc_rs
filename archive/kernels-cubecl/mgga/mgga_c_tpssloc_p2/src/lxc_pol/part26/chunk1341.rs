//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1341/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1341<F: Float>(t39054: F, t7245: F, t50: F, t9300: F, t10913: F, t1860: F, t1864: F, t2109: F, t2110: F, t22489: F, t22493: F, t22546: F, t24498: F, t24504: F, t24505: F, t24511: F, t6486: F, t6495: F, t6509: F, t67: F, t7246: F, t7251: F, t7255: F, t7256: F, t7259: F, t83699: F, t83706: F, t83710: F, t83771: F, t83803: F, t9258: F, t9288: F) -> F {
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85569 = F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t7246 * t83771 + t6495 * t24511 - F::cast_from(15.0_f64) * t85536 * t22546 - t1860 * (F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t85539 * t9288 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24498 * t10913 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7251 * t9258 + t83803) * t67 * t1864 / F::cast_from(6.0_f64) - t1860 * t24504 * t6509 / F::cast_from(2.0_f64) - t1860 * t7255 * t22489 / F::cast_from(2.0_f64) - t1860 * t2109 * t83706 / F::cast_from(6.0_f64) + t83699 * t2110 - t83710 * t2110 / F::cast_from(6.0_f64) - t22493 * t7256 / F::cast_from(2.0_f64) - t22493 * t7259 / F::cast_from(2.0_f64) - t6486 * t24505 / F::cast_from(2.0_f64);
    t85569
}
