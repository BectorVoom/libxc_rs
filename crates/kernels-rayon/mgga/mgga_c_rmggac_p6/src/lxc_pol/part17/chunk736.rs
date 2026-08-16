//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 736/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk736(t7186: f64, t7294: f64, t7299: f64, t7313: f64, t7326: f64, t7336: f64, t7346: f64, t7355: f64, t7387: f64, t7492: f64, t7559: f64, t7562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34521 = 0.44715219694310041527e-2_f64 * t7186;
    let t34544 = 0.24390119833260022651e-2_f64 * t7294;
    let t34545 = 0.5854811038705731867e-3_f64 * t7299;
    let t34548 = 0.91462949374725084942e-3_f64 * t7313;
    let t34551 = 0.10260057759007034251e-5_f64 * t7326;
    let t34554 = 0.45731474687362542471e-3_f64 * t7336;
    let t34557 = 0.45731474687362542471e-3_f64 * t7346;
    let t34558 = 0.13010691197123848594e-3_f64 * t7355;
    let t34567 = 0.45731474687362542471e-3_f64 * t7387;
    let t34592 = 0.91462949374725084942e-3_f64 * t7492;
    let t34612 = 0.13010691197123848594e-3_f64 * t7559;
    let t34613 = 0.10000709273223291967e0_f64 * t7562;
    (t34521, t34544, t34545, t34548, t34551, t34554, t34557, t34558, t34567, t34592, t34612, t34613)
}
