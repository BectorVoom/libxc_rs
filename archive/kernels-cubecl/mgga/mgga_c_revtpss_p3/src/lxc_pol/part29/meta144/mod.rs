//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk744;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk745;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta144<F: Float>(t3140: F, t342: F, t1034: F, t358: F, t360: F, t368: F, t335: F, t365: F, t1043: F, t373: F, t73: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3141, t3143, t3145, t3147, t3148, t3149, t3150, t3151) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk744::<F>(t3140, t342, t1034, t358, t360, t368, t335, t365, t1043);
        let (t3152, t3153) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk745::<F>(t3151, t373, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk746::<F>(t357);
    (t3141, t3143, t3145, t3147, t3148, t3149, t3150, t3151, t3152, t3153, t3154)
}
