//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3270/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3270(t4423: f64, t775: f64, t40791: f64, t5989: f64, t10890: f64, t5985: f64, t124: f64, t14586: f64, t14791: f64, t221: f64, t2730: f64, t36833: f64, t40782: f64, t40784: f64, t40792: f64, t4343: f64, t4362: f64, t4433: f64, t50446: f64, t50977: f64, t50982: f64, t51049: f64, t61234: f64, t800: f64) -> (f64, f64) {
    let t62080 = t4423 * t775;
    let t62089 = t40791 * t5989;
    let t62095 = t10890 * t5985;
    let t62101 = 0.4065600224742826258e-4_f64 * t50977 + 7.0_f64 / 72.0_f64 * t50982 + 0.30234122406223992295e0_f64 * t40782 + 0.1133779590233399711e0_f64 * t40784 - 0.68598428988911579156e-2_f64 * t4362 * t14791 * t14586 * t62080 + 0.51448821741683684367e-2_f64 * t4362 * t36833 * t14586 * t51049 + 35.0_f64 / 72.0_f64 * t62089 + t2730 * t800 * t124 * t61234 / 8.0_f64 - 35.0_f64 / 216.0_f64 * t62095 + 35.0_f64 / 72.0_f64 * t40792 - t50446 * t221 * t4433 * t4343;
    (t62080, t62101)
}
