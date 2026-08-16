//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 521/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk521(t2484: f64, t775: f64, t752: f64, t136: f64, t753: f64, t124: f64) -> (f64, f64, f64, f64) {
    let t2485 = t2484 * t775;
    let t2486 = t752 * t2485;
    let t2489 = 1.0_f64 / t753 / t136;
    let t2490 = t124 * t2489;
    (t2485, t2486, t2489, t2490)
}
