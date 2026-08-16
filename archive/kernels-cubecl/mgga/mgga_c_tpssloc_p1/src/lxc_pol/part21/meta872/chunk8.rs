//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3220/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220<F: Float>(t1396: F, t1398: F, t1404: F, t16546: F, t1852: F, t20149: F, t20186: F, t3932: F, t3946: F, t45584: F, t45588: F, t55417: F, t6471: F, t6483: F, t66961: F, t66964: F, t66967: F, t66993: F) -> F {
    let tv4rho42 = F::cast_from(2.0_f64) * t1396 * t20186 + F::cast_from(2.0_f64) * t20149 * t1404 + t1398 * (t55417 + t66961) + F::cast_from(4.0_f64) * t66964 + F::cast_from(2.0_f64) * t45584 + F::cast_from(2.0_f64) * t66967 + F::cast_from(2.0_f64) * t1852 * t16546 + t3932 * t6483 + F::cast_from(2.0_f64) * t45588 + t6471 * t3946 + t66993;
    tv4rho42
}
