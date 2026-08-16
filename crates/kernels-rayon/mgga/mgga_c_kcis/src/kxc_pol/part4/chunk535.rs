//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 535/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk535(t2662: f64, t189: f64, t850: f64, t851: f64, t2318: f64, t2321: f64, t2323: f64, t2327: f64, t2329: f64, t2331: f64, t843: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2663 = 1.0_f64 / t2662;
    let t2664 = t189 * t2663;
    let t2665 = t850 * t850;
    let t2666 = t2665 * t851;
    let t2675 = -0.78438333333333333333e0_f64 * t2318 + 0.15687666666666666667e1_f64 * t2321 + 0.68863333333333333333e0_f64 * t2323 + 0.14025833333333333333e0_f64 * t2327 + 0.28051666666666666667e0_f64 * t2329 + 0.17365833333333333333e0_f64 * t2331;
    let t2676 = t2675 * t851;
    let t2679 = t843 * t843;
    let t2680 = 1.0_f64 / t2679;
    let t2681 = t189 * t2680;
    let t2682 = t197 * t197;
    (t2663, t2664, t2665, t2666, t2675, t2676, t2679, t2680, t2681, t2682)
}
