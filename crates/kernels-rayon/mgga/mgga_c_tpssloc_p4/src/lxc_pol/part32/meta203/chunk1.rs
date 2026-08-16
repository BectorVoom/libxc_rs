//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 984/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk984(t5: f64, t1437: f64, t2240: f64, t3953: f64, t5385: f64, t5389: f64, t5445: f64, t605: f64, t86: f64, t112: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t5449 = piecewise3(t8, 0.0_f64, -8.0_f64 * t1437 * t3953 + 20.0_f64 * t2240 * t5389 + t5385 * t86 - 4.0_f64 * t5445 * t605);
    let t5450 = t5449 * t112;
    (t5449, t5450)
}
