//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 63/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk63(t143: f64, t2: f64, t24: f64, t92: f64, t91: f64, t146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150 = t143 * t2;
    let t151 = t24 * t150;
    let t152 = t92 * t151;
    let t153 = f64::sqrt(t152);
    let t154 = t91 * t153;
    let t157 = 3.0_f64 + t154 / 3.0_f64 + t146 / 3.0_f64;
    (t150, t151, t152, t153, t154, t157)
}
