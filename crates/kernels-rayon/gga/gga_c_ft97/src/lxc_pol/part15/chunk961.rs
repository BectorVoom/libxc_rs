//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 961/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk961(t1109: f64, t5049: f64, t200: f64, t5005: f64, t21249: f64, t694: f64, t21237: f64, t25: f64, t18132: f64, t4952: f64, t6: f64, t1127: f64, t5014: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t79439 = t1109 * t5049;
    let t79457 = t200 * t5005;
    let t79489 = t694 * t21249;
    let t79559 = t694 * t21237;
    let t79593 = t21237 * t25;
    let t79622 = t18132 * t6 * t4952;
    let t79629 = t5014 * t1127;
    (t79439, t79457, t79489, t79559, t79593, t79622, t79629)
}
