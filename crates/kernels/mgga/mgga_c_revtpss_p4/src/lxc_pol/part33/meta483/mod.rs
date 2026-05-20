//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta483<F: Float>(t25331: F, t7064: F, t1949: F, t785: F, t780: F, t2439: F, t212: F, t7048: F, t689: F, t7014: F, t887: F, t7049: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25352, t25353, t25355) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1763::<F>(t25331, t7064, t1949, t785, t780, t2439, t212, t7048, t689, t7014, t887, t7049, t786);
    (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25352, t25353, t25355)
}
