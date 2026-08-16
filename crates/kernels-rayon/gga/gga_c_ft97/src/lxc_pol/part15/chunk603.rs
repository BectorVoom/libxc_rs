//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 603/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk603(t2440: f64, t70: f64, t327: f64, t9570: f64, t1851: f64, t971: f64, t7773: f64, t89: f64, t921: f64, t1636: f64, t943: f64, t3020: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10915 = t70 * t2440;
    let t10916 = t327 * t9570;
    let t10969 = t971 * t1851;
    let t11043 = t89 * t7773 * t921;
    let t11076 = t89 * t1636 * t943;
    let t11160 = t3020 * t3070;
    (t10915, t10916, t10969, t11043, t11076, t11160)
}
