//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 723/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk723(t1198: f64, t3531: f64, t1188: f64, t3495: f64, t3497: f64, t1196: f64, t1179: f64, t3515: f64, t3520: f64, t3523: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3533 = 0.11696447245269292414e1_f64 * t3531 * t1198;
    let t3535 = t3495 * t3497 * t1188;
    let t3537 = 0.11696447245269292414e1_f64 * t1196 * t3535;
    let t3539 = t1179 * t3515 * t1188;
    let t3541 = 0.5848223622634646207e0_f64 * t1196 * t3539;
    let t3542 = t3520 * t3497;
    let t3543 = t3542 * t3523;
    let t3545 = 0.17315859105681463759e2_f64 * t1196 * t3543;
    let t3546 = 0.11111111111111111111e-1_f64 * t3356;
    let t3551 = t3546 - 0.55555555555555555556e-2_f64 * t3358 - 0.55555555555555555555e-2_f64 * t3365 + 0.16666666666666666667e-1_f64 * t3370 + 0.83333333333333333333e-2_f64 * t3374;
    (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551)
}
