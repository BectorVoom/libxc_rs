//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta566<F: Float>(t13272: F, t607: F, t10301: F, t1470: F, t2247: F, t4181: F, t4187: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F) -> (F, F, F, F, F, F, F) {
        let (t101230, t101237, t101240, t101243, t101448, t101451, t101453) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1890::<F>(t13272, t607, t10301, t1470, t2247, t4181, t4187, t94976, t1513, t94975, t28036, t94978);
    (t101230, t101237, t101240, t101243, t101448, t101451, t101453)
}
