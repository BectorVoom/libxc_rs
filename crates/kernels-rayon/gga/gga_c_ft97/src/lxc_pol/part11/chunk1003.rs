//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1003/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1003(t2174: f64, t8232: f64, t1882: f64, t9264: f64, t2101: f64, t2179: f64, t9290: f64, t12746: f64, t13140: f64, t144: f64, t1901: f64, t2178: f64, t2185: f64, t2210: f64, t2221: f64, t3434: f64, t379: f64, t40525: f64, t40700: f64, t40739: f64, t446: f64, t609: f64, t616: f64, t9115: f64, t9144: f64, t9284: f64, t9293: f64, t9311: f64, t9316: f64, t9438: f64, t9440: f64) -> f64 {
    let t40900 = t8232 * t2174;
    let t40905 = t1882 * t9264;
    let t40911 = t2101 * t2179;
    let t40916 = t1882 * t9290;
    let t40922 = -4.0_f64 / 3.0_f64 * t1901 * t2221 * t3434 * t40739 - 16.0_f64 / 9.0_f64 * t1901 * t9115 * t12746 * t40700 + 8.0_f64 / 3.0_f64 * t1901 * t2210 * t9438 * t9440 * t379 - 8.0_f64 * t1901 * t13140 * t2178 * t609 * t9284 - 8.0_f64 / 9.0_f64 * t40900 + 4.0_f64 * t446 * t144 * t40525 + 4.0_f64 / 9.0_f64 * t40905 - 4.0_f64 / 3.0_f64 * t1901 * t9144 * t9316 * t379 + 8.0_f64 / 3.0_f64 * t1901 * t40911 * t9311 * t379 - 8.0_f64 / 3.0_f64 * t40916 + 8.0_f64 * t446 * t2185 * t616 * t9293;
    t40922
}
