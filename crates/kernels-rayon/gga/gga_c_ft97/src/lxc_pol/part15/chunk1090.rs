//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1090/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1090(t16963: f64, t1901: f64, t2221: f64, t4454: f64, t4462: f64, t50781: f64, t64231: f64, t64255: f64, t64279: f64, t77644: f64, t77678: f64, t77719: f64, t77721: f64, t77752: f64, t9115: f64) -> f64 {
    let t87754 = -8.0_f64 / 9.0_f64 * t77644 + 112.0_f64 / 243.0_f64 * t50781 + 4.0_f64 / 3.0_f64 * t77678 + 2.0_f64 / 3.0_f64 * t1901 * t2221 * t16963 * t4462 + 4.0_f64 / 9.0_f64 * t1901 * t9115 * t16963 * t4454 + 8.0_f64 / 9.0_f64 * t64231 + 8.0_f64 / 27.0_f64 * t77719 + 4.0_f64 / 9.0_f64 * t77721 - 4.0_f64 / 9.0_f64 * t77752 - 16.0_f64 / 9.0_f64 * t64255 + 16.0_f64 / 9.0_f64 * t64279;
    t87754
}
