//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk710;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta128<F: Float>(t3143: F, t360: F, t368: F, t335: F, t365: F, t3141: F, t73: F, t357: F) -> (F, F, F, F, F, F, F, F) {
        let (t3144, t3145, t3147, t3148, t3149, t3150, t3153) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk710::<F>(t3143, t360, t368, t335, t365, t3141, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk711::<F>(t357);
    (t3144, t3145, t3147, t3148, t3149, t3150, t3153, t3154)
}
