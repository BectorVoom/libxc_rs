//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1475/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475(t3625: f64, t44250: f64, t6639: f64, t21439: f64, t3624: f64, t11249: f64, t6622: f64, t3682: f64, t6667: f64, t474: f64, t6593: f64, t3089: f64) -> (f64, f64, f64, f64, f64) {
    let t70809 = t3625 * t44250 * t6639;
    let t70819 = t21439 * t3624;
    let t70890 = t6622 * t11249;
    let t70942 = t6667 * t3682;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    (t70809, t70819, t70890, t70942, t70994)
}
