//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk861(t10036: f64, t10079: f64, t1310: f64, t16346: f64, t16347: f64, t16348: f64, t4744: f64, t6488: f64, t6492: f64, t6823: f64, t6827: f64, t6840: f64) -> f64 {
    let t16619 = t6488 - t6823 + t6827 + t16346 - t16347 - t16348 - 7.0_f64 * t10036 + 3.0_f64 / 2.0_f64 * t10079 + t6492 - t6840 + 3.0_f64 / 2.0_f64 * t1310 * t4744;
    t16619
}
