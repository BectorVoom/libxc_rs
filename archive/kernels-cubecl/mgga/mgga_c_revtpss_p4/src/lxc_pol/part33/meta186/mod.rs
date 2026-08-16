//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta186<F: Float>(t1082: F, t4757: F, t1089: F, t4905: F, t1651: F, t3291: F, t4772: F, t354: F, t357: F, t999: F, t4781: F, t3298: F, t378: F) -> (F, F, F, F, F, F, F) {
        let (t4961, t4964, t4967, t4970, t4976, t4977, t4980) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk881::<F>(t1082, t4757, t1089, t4905, t1651, t3291, t4772, t354, t357, t999, t4781, t3298, t378);
    (t4961, t4964, t4967, t4970, t4976, t4977, t4980)
}
