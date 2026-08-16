//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 693/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk693(t27059: f64, t5899: f64, t23616: f64, t23629: f64, t23650: f64, t27028: f64, t27032: f64, t27037: f64, t27041: f64, t27045: f64, t27049: f64, t27051: f64, t27055: f64) -> (f64, f64) {
    let t27060 = t5899 * t27059;
    let t27063 = t27028 / 6.0_f64 + t27032 / 3.0_f64 + t27037 / 3.0_f64 - 6.0_f64 * t27041 + 2.0_f64 / 3.0_f64 * t27045 - t27049 / 2.0_f64 - t27051 / 9.0_f64 + t27055 - t23616 / 12.0_f64 - t23629 / 3.0_f64 - 3.0_f64 * t27060 - t23650 / 18.0_f64;
    (t27060, t27063)
}
