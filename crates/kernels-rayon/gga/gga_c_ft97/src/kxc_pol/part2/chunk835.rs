//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 835/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk835(t12362: f64, t12365: f64, t12353: f64, t12359: f64, t12564: f64, t12568: f64, t13117: f64, t9166: f64, t9372: f64, t9373: f64, t9380: f64, t12571: f64) -> (f64, f64) {
    let t13119 = 4.0_f64 / 27.0_f64 * t12362;
    let t13120 = 2.0_f64 / 3.0_f64 * t12365;
    let t13122 = t9372 + t9373 - t9380 + 4.0_f64 * t12353 - t13117 + 22.0_f64 / 9.0_f64 * t12359 - t13119 - t9166 + t13120 - t12564 - 2.0_f64 / 3.0_f64 * t12568;
    let t13123 = 4.0_f64 / 9.0_f64 * t12571;
    (t13122, t13123)
}
