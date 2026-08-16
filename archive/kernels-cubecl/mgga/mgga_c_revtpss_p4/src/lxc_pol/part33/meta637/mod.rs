//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta637<F: Float>(t10301: F, t1470: F, t2247: F, t4181: F, t4187: F, t28019: F, t531: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F) -> (F, F, F, F, F, F, F) {
        let (t101237, t101240, t101243, t101417, t101451, t101454, t101455) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2086::<F>(t10301, t1470, t2247, t4181, t4187, t28019, t531, t1513, t94975, t28036, t94978, t25823, t4287);
    (t101237, t101240, t101243, t101417, t101451, t101454, t101455)
}
