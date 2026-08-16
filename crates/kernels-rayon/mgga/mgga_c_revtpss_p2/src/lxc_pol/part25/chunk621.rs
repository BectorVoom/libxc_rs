//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 621/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk621(t3523: f64, t3542: f64, t1196: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t459: f64) -> (f64, f64, f64, f64) {
    let t3543 = t3542 * t3523;
    let t3545 = 0.17315859105681463759e2_f64 * t1196 * t3543;
    let t3546 = 0.11111111111111111111e-1_f64 * t3356;
    let t3551 = t3546 - 0.55555555555555555556e-2_f64 * t3358 - 0.55555555555555555555e-2_f64 * t3365 + 0.16666666666666666667e-1_f64 * t3370 + 0.83333333333333333333e-2_f64 * t3374;
    let t3552 = t3551 * t459;
    (t3543, t3545, t3551, t3552)
}
