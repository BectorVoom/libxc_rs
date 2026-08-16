//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 895/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk895(t144: f64, t34948: f64, t34950: f64, t1060: f64, t574: f64, t7339: f64, t28: f64, t33155: f64, t33161: f64, t35118: f64, t35122: f64, t35127: f64, t35151: f64, t35157: f64, t35162: f64, t35166: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t35169 = t144 * t34948;
    let t35172 = t144 * t34950;
    let t35176 = t574 * t1060 * t7339;
    let t35179 = -t33155 - 2.0_f64 / 3.0_f64 * t446 * t35118 + 2.0_f64 / 3.0_f64 * t446 * t35122 + t446 * t35127 / 3.0_f64 + t89 * t28 * t35151 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t35157 + 2.0_f64 / 3.0_f64 * t446 * t35162 - t446 * t35166 / 3.0_f64 - t33161 - t446 * t35169 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t35172 - t446 * t35176 / 3.0_f64;
    (t35169, t35172, t35176, t35179)
}
