//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 917/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk917(t5037: f64, t626: f64, t701: f64, t5041: f64, t13647: f64, t3799: f64, t228: f64, t231: f64, t4995: f64, t625: f64, t3771: f64, t4947: f64, t9608: f64) -> (f64, f64, f64, f64, f64) {
    let t65853 = t701 * t626 * t5037;
    let t65860 = t701 * t626 * t5041;
    let t65862 = t3799 * t13647;
    let t65952 = t228 * t4995 * t625 * t231;
    let t66092 = t3771 * t4947 * t9608;
    (t65853, t65860, t65862, t65952, t66092)
}
