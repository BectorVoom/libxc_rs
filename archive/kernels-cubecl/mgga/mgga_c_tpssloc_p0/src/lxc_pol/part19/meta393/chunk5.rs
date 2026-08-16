//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1496/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496<F: Float>(t12513: F, t12537: F, t1396: F, t1398: F, t1404: F, t3: F, t39022: F, t39024: F, t39026: F, t39028: F, t3932: F, t3946: F, t45546: F, t45580: F, t580: F) -> F {
    let tv4rho40 = t3 * t45546 * t580 + F::cast_from(4.0_f64) * t12513 * t1404 + F::cast_from(4.0_f64) * t12537 * t1396 + t1398 * t45580 + F::cast_from(6.0_f64) * t3932 * t3946 + F::cast_from(4.0_f64) * t39022 + F::cast_from(12.0_f64) * t39024 + F::cast_from(12.0_f64) * t39026 + F::cast_from(4.0_f64) * t39028;
    tv4rho40
}
