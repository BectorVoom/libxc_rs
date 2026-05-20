//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 800/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk800<F: Float>(t1198: F, t3531: F, t1188: F, t3495: F, t3497: F, t1196: F, t1179: F, t3515: F, t3520: F, t3523: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3533 = F::cast_from(0.11696447245269292414e1_f64) * t3531 * t1198;
    let t3535 = t3495 * t3497 * t1188;
    let t3537 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t3535;
    let t3539 = t1179 * t3515 * t1188;
    let t3541 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t3539;
    let t3542 = t3520 * t3497;
    let t3543 = t3542 * t3523;
    let t3545 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t3543;
    let t3546 = F::cast_from(0.11111111111111111111e-1_f64) * t3356;
    let t3551 = t3546 - F::cast_from(0.55555555555555555556e-2_f64) * t3358 - F::cast_from(0.55555555555555555555e-2_f64) * t3365 + F::cast_from(0.16666666666666666667e-1_f64) * t3370 + F::cast_from(0.83333333333333333333e-2_f64) * t3374;
    (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551)
}
