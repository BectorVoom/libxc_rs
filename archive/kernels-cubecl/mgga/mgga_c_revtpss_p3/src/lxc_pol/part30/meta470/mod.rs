//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta470<F: Float>(t25331: F, t7064: F, t1949: F, t785: F, t780: F, t2439: F, t212: F, t7048: F, t689: F, t231: F, t836: F, t7076: F) -> (F, F, F, F, F, F, F, F) {
        let (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25344) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1778::<F>(t25331, t7064, t1949, t785, t780, t2439, t212, t7048, t689, t231, t836, t7076);
    (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25344)
}
