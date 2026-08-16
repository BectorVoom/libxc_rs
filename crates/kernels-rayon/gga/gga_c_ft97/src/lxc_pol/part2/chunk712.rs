//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 712/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk712(t1710: f64, t3099: f64, t371: f64, t7876: f64, t1630: f64, t929: f64, t25: f64, t78: f64, t1602: f64, t122: f64, t1593: f64, t1664: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11225 = t1710 * t3099;
    let t11232 = t371 * t7876;
    let t11233 = t1630 * t929;
    let t11240 = t78 * t25;
    let t11241 = t1602 * t11240;
    let t11245 = t78 * t122;
    let t11246 = t1602 * t11245;
    let t11247 = t1593 * t929;
    let t11251 = t1664 * t939;
    (t11225, t11232, t11233, t11241, t11246, t11247, t11251)
}
