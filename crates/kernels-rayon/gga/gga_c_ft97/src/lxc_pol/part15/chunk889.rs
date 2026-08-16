//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 889/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk889(t2360: f64, t2842: f64, t309: f64, t43917: f64, t192: f64, t33828: f64, t43833: f64, t870: f64, t9570: f64, t313: f64, t41743: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44204 = t2842 * t2360;
    let t44245 = t43917 * t309;
    let t44280 = t192 * t33828;
    let t44335 = t43833 * t309;
    let t44340 = t870 * t9570;
    let t44436 = 280.0_f64 / 243.0_f64 * t89 * t41743 * t313;
    (t44204, t44245, t44280, t44335, t44340, t44436)
}
