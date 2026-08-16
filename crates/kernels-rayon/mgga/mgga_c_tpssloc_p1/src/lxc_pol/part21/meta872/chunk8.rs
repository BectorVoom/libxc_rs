//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3220/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220(t1396: f64, t1398: f64, t1404: f64, t16546: f64, t1852: f64, t20149: f64, t20186: f64, t3932: f64, t3946: f64, t45584: f64, t45588: f64, t55417: f64, t6471: f64, t6483: f64, t66961: f64, t66964: f64, t66967: f64, t66993: f64) -> f64 {
    let tv4rho42 = 2.0_f64 * t1396 * t20186 + 2.0_f64 * t20149 * t1404 + t1398 * (t55417 + t66961) + 4.0_f64 * t66964 + 2.0_f64 * t45584 + 2.0_f64 * t66967 + 2.0_f64 * t1852 * t16546 + t3932 * t6483 + 2.0_f64 * t45588 + t6471 * t3946 + t66993;
    tv4rho42
}
