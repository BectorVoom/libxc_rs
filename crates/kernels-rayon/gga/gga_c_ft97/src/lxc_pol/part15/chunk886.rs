//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 886/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk886(t191: f64, t33828: f64, t10: f64, t11175: f64, t296: f64, t190: f64, t2680: f64, t305: f64, t36452: f64, t37991: f64, t11176: f64, t303: f64) -> (f64, f64, f64, f64, f64) {
    let t43524 = t191 * t33828;
    let t43537 = t10 * t11175 * t296;
    let t43538 = 280.0_f64 / 243.0_f64 * t43537;
    let t43548 = 1.0_f64 / t305 / t37991 / t190 / t2680 / t36452 / 96.0_f64;
    let t43574 = 280.0_f64 / 81.0_f64 * t11176 * t303;
    (t43524, t43537, t43538, t43548, t43574)
}
