//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 558/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk558(t64: f64, t7866: f64, t2037: f64, t428: f64, t25: f64, t409: f64, t1602: f64) -> (f64, f64, f64, f64) {
    let t7867 = t64 * t7866;
    let t7868 = t2037 * t428;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    (t7867, t7868, t7876, t7877)
}
