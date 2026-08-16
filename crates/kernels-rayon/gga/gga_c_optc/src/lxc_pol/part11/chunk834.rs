//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 834/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk834(t43: f64, t16225: f64, t16231: f64, t3365: f64, t4565: f64, t47: f64, t6713: f64, t1239: f64, t4570: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t16235 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t6713 * t16225 + 4.0_f64 / 3.0_f64 * t3365 * t4565 + 4.0_f64 / 3.0_f64 * t47 * t16231);
    let t16236 = t4570 * t1239;
    (t16235, t16236)
}
