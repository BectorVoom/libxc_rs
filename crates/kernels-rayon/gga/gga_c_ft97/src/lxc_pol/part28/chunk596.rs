//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 596/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk596(t25621: f64, t28: f64, t22563: f64, t929: f64, t7983: f64, t22718: f64, t6427: f64, t22701: f64, t938: f64, t3099: f64, t5522: f64, t428: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25622 = t28 * t25621;
    let t25625 = t22563 * t929;
    let t25626 = t7983 * t25625;
    let t25631 = t22718 * t6427;
    let t25637 = t22701 * t938;
    let t25640 = t5522 * t3099;
    let t25643 = t938 * t428;
    (t25622, t25626, t25631, t25637, t25640, t25643)
}
