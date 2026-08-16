//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 636/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk636(t28255: f64, t729: f64, t762: f64, t242: f64, t27984: f64, t2574: f64, t265: f64, t27878: f64, t6837: f64, t766: f64, t2469: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28257 = t729 * t762 * t28255;
    let t28260 = t242 * t27984;
    let t28264 = t2574 * t265 * t27878;
    let t28267 = t6837 * t766;
    let t28269 = t729 * t762 * t28267;
    let t28273 = t729 * t2469 * t6861;
    (t28257, t28260, t28264, t28267, t28269, t28273)
}
