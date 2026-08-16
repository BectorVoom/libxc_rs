//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 585/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk585<F: Float>(t2672: F, t2686: F, t2691: F, t2730: F, t4359: F, t4373: F, t4455: F, t5980: F, t5985: F, t5989: F, t5993: F, t6040: F, t799: F, t825: F, t851: F) -> F {
    let t6041 = -F::cast_from(0.21437009059034868486e-3_f64) * t825 * t5980 + F::cast_from(0.20007875121765877254e-2_f64) * t4359 - t799 * t5985 / F::cast_from(48.0_f64) + t2730 * t5989 / F::cast_from(16.0_f64) + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t5993 - t2672 + t2686 + F::cast_from(0.57165357490759649296e-4_f64) * t4373 + t2691 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4455 + t6040;
    t6041
}
