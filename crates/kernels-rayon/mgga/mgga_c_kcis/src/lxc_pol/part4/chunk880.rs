//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 880/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk880(t2026: f64, t3393: f64, t3751: f64, t538: f64, t5427: f64, t1517: f64, t1650: f64, t4225: f64, t1392: f64, t5441: f64, t1518: f64, t167: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5966 = t3393 * t2026;
    let t5968 = t3751 * t538;
    let t5969 = t5968 * t5427;
    let t5973 = t1517 * t4225 * t1650;
    let t5976 = t1392 * t538;
    let t5977 = t5976 * t5441;
    let t5981 = t1517 * t1518 * t167;
    (t5966, t5968, t5969, t5973, t5976, t5977, t5981)
}
