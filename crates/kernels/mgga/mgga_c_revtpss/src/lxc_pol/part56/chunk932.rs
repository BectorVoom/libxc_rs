//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 932/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk932<F: Float>(t1399: F, t32195: F, t5673: F, t32194: F, t1955: F, t843: F, t8571: F, t8575: F, t239: F, t240: F, t31752: F, t545: F) -> (F, F, F, F) {
    let t32197 = t5673 * t32195 * t1399;
    let t32198 = t32194 * t32197;
    let t32202 = t1955 * t8571 * t843 * t8575;
    let t32203 = F::new(0.131760844872908846e-2) * t32202;
    let t32206 = t31752 * t545 * t239 * t240;
    (t32197, t32198, t32203, t32206)
}
