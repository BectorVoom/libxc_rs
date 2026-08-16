//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 595/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk595(t237: f64, t27703: f64, t17859: f64, t24260: f64, t1100: f64, t1416: f64, t1113: f64, t218: f64, t709: f64, t24345: f64) -> (f64, f64, f64, f64, f64) {
    let t27704 = t27703 * t237;
    let t27707 = t24260 * t17859;
    let t27711 = t1100 * t1416;
    let t27712 = t218 * t1113;
    let t27713 = t27712 * t709;
    let t27717 = t1100 * t24345;
    (t27704, t27707, t27711, t27713, t27717)
}
