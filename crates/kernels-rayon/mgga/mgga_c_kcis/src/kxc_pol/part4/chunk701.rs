//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 701/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk701(t3815: f64, t4035: f64, t1409: f64, t3841: f64, t1075: f64, t317: f64, t522: f64, t3106: f64, t323: f64, t526: f64, t3110: f64, t534: f64) -> (f64, f64, f64, f64, f64) {
    let t4036 = t4035 * t3815;
    let t4039 = t1409 * t3841;
    let t4047 = 0.8197e-2_f64 * t317 * t1075 * t522;
    let t4050 = 0.21133333333333333333e-2_f64 * t323 * t3106 * t526;
    let t4051 = t3110 * t534;
    (t4036, t4039, t4047, t4050, t4051)
}
