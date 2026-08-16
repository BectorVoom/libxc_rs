//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 511/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk511(t1265: f64, t370: f64, t1273: f64, t1275: f64, t4007: f64, t4060: f64, t4011: f64, t4015: f64, t4018: f64, t4021: f64, t4039: f64, t4047: f64, t4055: f64, t4057: f64, t4063: f64, t4067: f64, t4070: f64, t4073: f64) -> (f64, f64, f64, f64) {
    let t4099 = t1265 * t370;
    let t4100 = 1.0_f64 / t4099;
    let t4101 = t1273 * t1273;
    let t4103 = t4100 * t4101 * t1275;
    let t4108 = 0.40256666666666666667e0_f64 * t4007;
    let t4115 = 0.27595e0_f64 * t4060;
    let t4120 = -0.1294625e1_f64 * t4039 + 0.258925e1_f64 * t4047 + t4108 + 0.20128333333333333334e0_f64 * t4011 - 0.20128333333333333333e0_f64 * t4015 + 0.60385e0_f64 * t4018 - 0.301925e0_f64 * t4021 + 0.82524375e-1_f64 * t4055 + 0.16504875e0_f64 * t4057 + t4115 + 0.22076e0_f64 * t4063 - 0.5519e-1_f64 * t4067 + 0.33114e0_f64 * t4070 - 0.16557e0_f64 * t4073;
    (t4100, t4101, t4103, t4120)
}
