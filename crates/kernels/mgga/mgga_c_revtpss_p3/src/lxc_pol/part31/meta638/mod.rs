//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta638<F: Float>(t2247: F, t4181: F, t4187: F, t60221: F, t6957: F, t13272: F, t25105: F, t10309: F, t28126: F, t60224: F, t10301: F, t28076: F, t38: F) -> (F, F, F, F, F, F, F, F) {
        let (t101240, t101243, t101320, t101326, t101333, t101342, t101385, t101391) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2094::<F>(t2247, t4181, t4187, t60221, t6957, t13272, t25105, t10309, t28126, t60224, t10301, t28076, t38);
    (t101240, t101243, t101320, t101326, t101333, t101342, t101385, t101391)
}
