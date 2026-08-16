//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1122/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1122(t21193: f64, t3799: f64, t41477: f64, t420: f64, t701: f64, t88252: f64, t2446: f64, t88239: f64, t18043: f64, t5042: f64, t21201: f64, t1124: f64, t79802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88562 = t3799 * t21193;
    let t88566 = t701 * t420 * t41477 * t88252;
    let t88570 = t701 * t420 * t2446 * t88239;
    let t88572 = t18043 * t5042;
    let t88575 = t3799 * t21201;
    let t88577 = t79802 * t1124;
    (t88562, t88566, t88570, t88572, t88575, t88577)
}
