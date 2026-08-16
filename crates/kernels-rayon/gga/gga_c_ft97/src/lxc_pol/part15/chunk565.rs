//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 565/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk565(t7954: f64, t82: f64, t177: f64, t2280: f64, t1736: f64, t70: f64, t179: f64, t7763: f64, t11: f64, t2247: f64) -> (f64, f64, f64, f64, f64) {
    let t8577 = t7954 * t82;
    let t8618 = 1.0_f64 / t2280 / t177;
    let t8633 = t70 * t1736;
    let t8634 = t179 * t7763;
    let t8639 = t11 * t2247;
    (t8577, t8618, t8633, t8634, t8639)
}
