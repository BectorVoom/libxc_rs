//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 833/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk833(t1882: f64, t3467: f64, t12606: f64, t144: f64, t1053: f64, t1986: f64, t2185: f64, t605: f64, t12306: f64, t12308: f64, t12310: f64, t12285: f64, t12290: f64, t12293: f64, t12296: f64, t12300: f64, t12304: f64, t12315: f64, t12881: f64) -> (f64, f64, f64, f64) {
    let t13084 = 2.0_f64 / 27.0_f64 * t1882 * t3467;
    let t13085 = t144 * t12606;
    let t13088 = t1053 * t1986;
    let t13090 = t2185 * t605 * t13088;
    let t13100 = 2.0_f64 / 9.0_f64 * t12306;
    let t13101 = 4.0_f64 / 9.0_f64 * t12308;
    let t13102 = 4.0_f64 / 27.0_f64 * t12310;
    let t13104 = t12881 / 2.0_f64 + t12285 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t12290 - 10.0_f64 / 27.0_f64 * t12293 - 8.0_f64 / 9.0_f64 * t12296 + t12300 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t12304 - t13100 - t13101 + t13102 - 2.0_f64 / 3.0_f64 * t12315;
    (t13084, t13085, t13090, t13104)
}
