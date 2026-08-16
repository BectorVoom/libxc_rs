//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 822/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk822(t12362: f64, t12365: f64, t12353: f64, t12359: f64, t12564: f64, t12568: f64, t12911: f64, t8799: f64, t8802: f64, t9059: f64, t9383: f64, t12571: f64) -> (f64, f64) {
    let t12913 = 4.0_f64 / 81.0_f64 * t12362;
    let t12914 = 2.0_f64 / 9.0_f64 * t12365;
    let t12917 = t8799 / 27.0_f64 + 2.0_f64 / 81.0_f64 * t8802 - 2.0_f64 / 27.0_f64 * t9059 + 4.0_f64 / 3.0_f64 * t12353 - t12911 + 22.0_f64 / 27.0_f64 * t12359 - t12913 - t9383 + t12914 - t12564 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t12568;
    let t12918 = 4.0_f64 / 27.0_f64 * t12571;
    (t12917, t12918)
}
