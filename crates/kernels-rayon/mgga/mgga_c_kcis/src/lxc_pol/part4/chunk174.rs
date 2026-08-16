//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 174/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk174(t317: f64, t318: f64, t323: f64, t324: f64, t333: f64, t334: f64, t510: f64, t522: f64, t526: f64, t532: f64, t534: f64) -> f64 {
    let t538 = -0.11955719325063177623e-1_f64 * t510 + 0.263475e-2_f64 * t317 * t318 * t522 - 0.4755e-3_f64 * t323 * t324 * t526 + 0.2589769453898153438e-4_f64 * t532 - 0.21605625e-5_f64 * t333 * t334 * t534;
    t538
}
