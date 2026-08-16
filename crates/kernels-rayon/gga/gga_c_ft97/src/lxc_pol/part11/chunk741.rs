//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 741/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk741(t241: f64, t9568: f64, t265: f64, t9572: f64, t2373: f64, t766: f64, t2574: f64, t762: f64, t2569: f64, t713: f64, t2568: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10024 = t9568 * t241;
    let t10026 = t10024 * t265 * t9572;
    let t10029 = t2373 * t766;
    let t10031 = t2574 * t762 * t10029;
    let t10034 = t2569 * t713;
    let t10036 = t729 * t2568 * t10034;
    (t10024, t10026, t10029, t10031, t10034, t10036)
}
