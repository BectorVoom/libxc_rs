//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 537/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk537(t295: f64, t312: f64, t4239: f64, t1901: f64, t193: f64, t2839: f64, t2872: f64, t3281: f64, t4142: f64, t4147: f64, t4152: f64, t4156: f64, t4159: f64, t4164: f64, t4169: f64, t4173: f64, t4178: f64, t4183: f64, t4188: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t4241 = t295 * t4239 * t312;
    let t4245 = -2.0_f64 / 27.0_f64 * t1901 * t4142 + t1901 * t4147 / 9.0_f64 + t1901 * t4152 / 9.0_f64 + t2872 / 27.0_f64 + t4156 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t3281 * t4159 + 2.0_f64 / 3.0_f64 * t446 * t4164 + t446 * t4169 / 3.0_f64 - t446 * t4173 / 9.0_f64 + t446 * t4178 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t4183 - t2839 / 9.0_f64 - t446 * t4188 / 9.0_f64 + t89 * t193 * t4241 / 3.0_f64;
    (t4241, t4245)
}
