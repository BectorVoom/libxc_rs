//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta597<F: Float>(t2022: F, t22: F, t25937: F, t94696: F, t7282: F, t93139: F, t1955: F, t25920: F, t4075: F, t2435: F, t26061: F, t1385: F, t7274: F, t2453: F, t26053: F, t9676: F, t1358: F, t2439: F, t785: F, t26064: F, t3920: F, t1426: F, t7275: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t94700, t94703, t94705, t94714, t94716) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2017::<F>(t2022, t22, t25937, t94696, t7282, t93139, t1955, t25920, t4075, t2435, t26061, t1385, t7274);
        let (t94725, t94726, t94733, t94735, t94748) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2018::<F>(t2453, t26053, t9676, t1358, t2439, t7274, t785, t26064, t3920, t1426, t7275, t786);
    (t94700, t94703, t94705, t94714, t94716, t94725, t94726, t94733, t94735, t94748)
}
