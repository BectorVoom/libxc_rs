//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 650/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk650(t3523: f64, t6555: f64, t1196: f64, t3546: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t459: f64) -> (f64, f64, f64, f64) {
    let t6556 = t6555 * t3523;
    let t6558 = 0.17315859105681463759e2_f64 * t1196 * t6556;
    let t6563 = t3546 - 0.55555555555555555556e-2_f64 * t5044 - 0.55555555555555555555e-2_f64 * t6423 + 0.16666666666666666667e-1_f64 * t6427 + 0.83333333333333333333e-2_f64 * t6431;
    let t6564 = t6563 * t459;
    (t6556, t6558, t6563, t6564)
}
