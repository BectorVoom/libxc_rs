//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 619/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk619(t173: f64, t2440: f64, t420: f64, t9651: f64, t1103: f64, t228: f64, t231: f64, t625: f64, t1123: f64, t626: f64, t701: f64, t1152: f64, t1771: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13598 = t173 * t2440;
    let t13605 = t420 * t9651;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13680 = t1771 * t1152;
    (t13598, t13605, t13643, t13647, t13648, t13680)
}
