//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 571/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk571<F: Float>(t300: F, t3527: F, t3489: F, t1175: F, t1198: F, t1188: F, t3495: F, t3497: F, t1196: F, t1179: F, t3515: F, t3520: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3528 = t300 * t3527;
    let t3530 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t3489;
    let t3531 = t300 * t1175;
    let t3533 = F::cast_from(0.11696447245269292414e1_f64) * t3531 * t1198;
    let t3535 = t3495 * t3497 * t1188;
    let t3537 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t3535;
    let t3539 = t1179 * t3515 * t1188;
    let t3541 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t3539;
    let t3542 = t3520 * t3497;
    (t3528, t3530, t3531, t3533, t3535, t3537, t3539, t3541, t3542)
}
