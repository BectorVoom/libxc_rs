//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 963/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk963(t18034: f64, t3799: f64, t173: f64, t21182: f64, t701: f64, t21200: f64, t18043: f64, t3803: f64, t18031: f64, t21192: f64, t21210: f64, t227: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t79786 = t3799 * t18034;
    let t79789 = t701 * t173 * t21182;
    let t79792 = t701 * t173 * t21200;
    let t79794 = t18043 * t3803;
    let t79796 = t3799 * t18031;
    let t79799 = t701 * t173 * t21192;
    let t79802 = t9 * t227 * t21210;
    (t79786, t79789, t79792, t79794, t79796, t79799, t79802)
}
