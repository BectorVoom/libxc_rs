//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 853/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk853(t13309: f64, t3910: f64, t2: f64, t9952: f64, t9570: f64, t992: f64, t2349: f64) -> (f64, f64, f64) {
    let t13310 = t3910 * t13309;
    let t13313 = t9952 * t2;
    let t13314 = t9570 * t992;
    let t13315 = t13314 * t2349;
    (t13310, t13313, t13315)
}
