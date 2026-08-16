//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 999/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk999(t4527: f64, t908: f64, t2861: f64, t5027: f64, t5030: f64, t1094: f64, t4922: f64, t1775: f64, t9528: f64, t341: f64, t9368: f64, t1017: f64, t86: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13095 = t4527 * t908;
    let t13096 = 2.0_f64 * t13095;
    let t13101 = t2861 * t5027;
    let t13102 = 0.33163888888888888888e-2_f64 * t13101;
    let t13103 = t2861 * t5030;
    let t13105 = t4922 * t1094;
    let t13106 = t13105 * sigma0;
    let t13122 = t9528 * t1775;
    let t13128 = t9368 * t341;
    let t13130 = t86 * t1017 * t13128;
    (t13096, t13101, t13102, t13103, t13105, t13106, t13122, t13130)
}
