//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 451/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk451(t3213: f64, t3288: f64, t103: f64, t3170: f64, t108: f64, t2976: f64, t3109: f64, t3220: f64, t3239: f64, t3256: f64, t3262: f64, t438: f64, t497: f64, t88: f64, t948: f64, t984: f64) -> (f64, f64, f64) {
    let t3289 = t3213 + t3288;
    let t3291 = t3170 * t103;
    let t3297 = -t108 * t2976 - t108 * t3109 - t3289 * t88 - t438 * t984 - t497 * t948 + 4.0_f64 * t3220 - 2.0_f64 * t3239 - 2.0_f64 * t3256 - 2.0_f64 * t3262 + 2.0_f64 * t3291;
    (t3289, t3291, t3297)
}
