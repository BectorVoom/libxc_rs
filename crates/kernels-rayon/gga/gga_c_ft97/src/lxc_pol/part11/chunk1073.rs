//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1073/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1073(t42009: f64, t42025: f64, t42042: f64, t42250: f64, t626: f64, t703: f64, t1526: f64, t2322: f64, t2355: f64, t9483: f64, t9503: f64, t13598: f64, t9491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42252 = t42009 + t42025 + t42042 + t42250;
    let t42262 = t626 * t703;
    let t42264 = t1526 * t42262 * t2322;
    let t42267 = t1526 * t9483 * t2355;
    let t42270 = t1526 * t9483 * t9503;
    let t42273 = t1526 * t13598 * t9491;
    (t42252, t42262, t42264, t42267, t42270, t42273)
}
