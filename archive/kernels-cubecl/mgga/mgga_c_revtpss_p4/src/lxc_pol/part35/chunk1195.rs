//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1195/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1195<F: Float>(t108516: F, t108524: F, t108537: F, t108539: F, t108554: F, t108559: F, t108562: F, t114521: F, t114525: F, t114527: F, t98141: F, t98148: F, t98161: F, t98165: F) -> F {
    let t115027 = -F::cast_from(0.96037800584476210818e-1_f64) * t108516 + F::cast_from(0.12196800674228478774e-2_f64) * t108524 + F::cast_from(0.10289764348336736873e-1_f64) * t114521 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t108537 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t108539 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t114525 + F::cast_from(0.10289764348336736873e-1_f64) * t114527 - F::cast_from(0.17149607247227894789e-2_f64) * t108554 - F::cast_from(0.91464571985215438874e-3_f64) * t98141 + F::cast_from(0.65049603595885220128e-2_f64) * t98148 + F::cast_from(0.30492001685571196935e-4_f64) * t98161 - F::cast_from(0.68598428988911579154e-3_f64) * t108559 + F::cast_from(0.30492001685571196935e-3_f64) * t108562 - F::cast_from(0.27210710165601593065e0_f64) * t98165;
    t115027
}
