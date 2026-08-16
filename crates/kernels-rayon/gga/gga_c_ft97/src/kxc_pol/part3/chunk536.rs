//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 536/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk536(t4226: f64, t845: f64, t91: f64, t4032: f64, t4049: f64, t2656: f64, t2659: f64, t2823: f64, t4035: f64, t4039: f64, t4042: f64, t4046: f64, t4054: f64, t4059: f64, t4132: f64, t4193: f64) -> (f64, f64, f64, f64) {
    let t4228 = t91 * t845 * t4226;
    let t4230 = t4032 / 27.0_f64;
    let t4235 = t4049 / 9.0_f64;
    let t4239 = -t4193 / 12.0_f64 + t4228 / 6.0_f64 + t2823 + t2656 + t2659 + t4230 - 2.0_f64 / 27.0_f64 * t4035 + t4039 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t4042 + 2.0_f64 / 9.0_f64 * t4046 + t4235 + t4054 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t4059 - t4132 / 3.0_f64;
    (t4228, t4230, t4235, t4239)
}
