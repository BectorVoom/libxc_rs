//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 603/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk603(t1130: f64, t330: f64, t1133: f64, t829: f64, t3210: f64, t3200: f64, t388: f64) -> (f64, f64, f64, f64, f64) {
    let t3211 = t1130 * t330;
    let t3212 = t829 * t1133;
    let t3213 = t3211 * t3212;
    let t3214 = t3210 * t3213;
    let t3215 = t3200 * t3214;
    let t3217 = 1.0_f64 / t388;
    (t3211, t3212, t3214, t3215, t3217)
}
