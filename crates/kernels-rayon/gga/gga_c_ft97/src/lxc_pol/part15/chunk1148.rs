//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1148/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1148(t21531: f64, t51340: f64, t1091: f64, t14159: f64, t18467: f64, t1901: f64, t21646: f64, t21673: f64, t21753: f64, t2599: f64, t2606: f64, t3892: f64, t42517: f64, t51990: f64, t80334: f64, t80460: f64, t81365: f64, t81411: f64, t81448: f64, t81454: f64, t81469: f64, t89222: f64) -> (f64, f64) {
    let t89371 = t51340 * t21531;
    let t89404 = -8.0_f64 / 3.0_f64 * t81365 - 8.0_f64 / 9.0_f64 * t81411 + 8.0_f64 / 3.0_f64 * t1901 * t2599 * t3892 * t89222 + 4.0_f64 / 3.0_f64 * t1901 * t14159 * t21646 + 8.0_f64 / 3.0_f64 * t1901 * t42517 * t80460 * t1091 + 8.0_f64 / 9.0_f64 * t1901 * t18467 * t21753 + 8.0_f64 / 9.0_f64 * t1901 * t51990 * t21673 + 4.0_f64 / 9.0_f64 * t1901 * t2606 * t80334 * t1091 - 8.0_f64 / 9.0_f64 * t81448 + 4.0_f64 / 9.0_f64 * t81454 - 4.0_f64 / 9.0_f64 * t81469;
    (t89371, t89404)
}
