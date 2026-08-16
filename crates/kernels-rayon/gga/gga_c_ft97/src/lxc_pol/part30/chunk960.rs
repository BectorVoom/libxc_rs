//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 960/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk960(t1526: f64, t6335: f64, t9483: f64, t1491: f64, t7570: f64, t8281: f64, t34284: f64, t34287: f64, t25462: f64, t34003: f64, t33998: f64, t25485: f64, t7581: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142566 = t1526 * t9483 * t6335;
    let t142576 = 2.0_f64 / 27.0_f64 * t7570 * t8281 * t1491;
    let t142577 = t34284 * t34287;
    let t142595 = t25462 * t34003;
    let t142597 = t25462 * t33998;
    let t142602 = 2.0_f64 / 27.0_f64 * t7581 * t25485;
    (t142566, t142576, t142577, t142595, t142597, t142602)
}
