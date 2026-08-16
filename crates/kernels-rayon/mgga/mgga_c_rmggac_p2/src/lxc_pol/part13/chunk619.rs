//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 619/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk619(t8141: f64, t8158: f64, t515: f64, t235: f64, t7579: f64, t7678: f64, t7680: f64, t7683: f64, t7685: f64, t7688: f64, t7692: f64, t7697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8159 = t8141 + t8158;
    let t8160 = t515 * t8159;
    let t8161 = t235 * t8160;
    let t8162 = 0.19957069503106347607e-1_f64 * t8161;
    let t8163 = 0.5987120850931904282e-1_f64 * t7579;
    let t8164 = 0.85129199786595678799e-5_f64 * t7678;
    let t8166 = 0.5107751987195740728e-4_f64 * t7680;
    let t8167 = 0.2553875993597870364e-4_f64 * t7683;
    let t8168 = 0.1702583995731913576e-4_f64 * t7685;
    let t8169 = 0.85129199786595678799e-5_f64 * t7688;
    let t8170 = 0.212822999466489197e-4_f64 * t7692;
    let t8171 = 0.1064114997332445985e-4_f64 * t7697;
    (t8159, t8160, t8162, t8163, t8164, t8166, t8167, t8168, t8169, t8170, t8171)
}
