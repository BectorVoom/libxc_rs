//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 903/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk903(t2537: f64, t779: f64, t2539: f64, t2728: f64, t887: f64, t2751: f64, t2489: f64, t747: f64, t2492: f64, t752: f64, t753: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8522 = t779 * t2537;
    let t8523 = t8522 * t2539;
    let t8524 = 6.0_f64 * t8523;
    let t8525 = t887 * t2728;
    let t8526 = t8525 * t2751;
    let t8531 = t747 * t2489;
    let t8532 = t8531 * t2492;
    let t8533 = t752 * t8532;
    let t8535 = t753 * t753;
    let t8536 = 1.0_f64 / t8535;
    let t8537 = t124 * t8536;
    (t8524, t8526, t8531, t8533, t8536, t8537)
}
