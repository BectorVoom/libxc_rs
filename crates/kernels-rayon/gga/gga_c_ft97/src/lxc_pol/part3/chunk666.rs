//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 666/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk666(t195: f64, t9606: f64, t25: f64, t209: f64, t2247: f64, t228: f64, t231: f64, t626: f64, t705: f64, t701: f64, t191: f64, t2360: f64) -> (f64, f64, f64, f64, f64) {
    let t9608 = 1.0_f64 / t195 / t9606;
    let t9609 = t25 * t9608;
    let t9634 = t209 * t2247;
    let t9636 = t228 * t9634 * t231;
    let t9637 = 0.70937342644032921812e-2_f64 * t9636;
    let t9638 = t626 * t705;
    let t9639 = t701 * t9638;
    let t9651 = 1.0_f64 / t191 / t2360;
    (t9609, t9636, t9637, t9639, t9651)
}
