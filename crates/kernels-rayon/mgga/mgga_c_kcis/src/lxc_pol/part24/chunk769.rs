//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 769/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk769(t10470: f64, t361: f64, t1127: f64, t3245: f64, t1138: f64, t3329: f64, t1140: f64, t364: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10471 = t10470 * t361;
    let t10472 = 0.73697530864197530862e-3_f64 * t10471;
    let t10473 = t3245 * t1127;
    let t10491 = t1138 * t3329;
    let t10496 = t1140 * t1140;
    let t10497 = 1.0_f64 / t10496;
    let t10498 = t364 * t10497;
    (t10471, t10472, t10473, t10491, t10496, t10497, t10498)
}
