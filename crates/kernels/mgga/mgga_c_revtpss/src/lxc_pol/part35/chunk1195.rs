//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1195/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1195<F: Float>(t108516: F, t108524: F, t108537: F, t108539: F, t108554: F, t108559: F, t108562: F, t114521: F, t114525: F, t114527: F, t98141: F, t98148: F, t98161: F, t98165: F) -> F {
    let t115027 = -F::new(0.96037800584476210818e-1) * t108516 + F::new(0.12196800674228478774e-2) * t108524 + F::new(0.10289764348336736873e-1) * t114521 + F::new(7.0) / F::new(24.0) * t108537 - F::new(7.0) / F::new(8.0) * t108539 + F::new(3.0) / F::new(8.0) * t114525 + F::new(0.10289764348336736873e-1) * t114527 - F::new(0.17149607247227894789e-2) * t108554 - F::new(0.91464571985215438874e-3) * t98141 + F::new(0.65049603595885220128e-2) * t98148 + F::new(0.30492001685571196935e-4) * t98161 - F::new(0.68598428988911579154e-3) * t108559 + F::new(0.30492001685571196935e-3) * t108562 - F::new(0.27210710165601593065e0) * t98165;
    t115027
}
