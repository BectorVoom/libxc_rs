//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2042;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta586<F: Float>(t2453: F, t26053: F, t9676: F, t4078: F, t689: F, t7242: F, t1358: F, t2439: F, t7274: F, t785: F, t26064: F, t3920: F, t1426: F, t7275: F, t786: F, t3917: F, t25953: F, t26072: F, t2435: F, t25913: F, t7289: F, t94600: F, t2028: F, t3999: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94725, t94726, t94729, t94733, t94735) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2042::<F>(t2453, t26053, t9676, t4078, t689, t7242, t1358, t2439, t7274, t785, t26064, t3920);
        let (t94748, t94749, t94756, t94758, t94761, t94762) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2043::<F>(t1426, t7275, t786, t3917, t25953, t26072, t2435, t25913, t7289, t94600, t2028, t3999);
    (t94725, t94726, t94729, t94733, t94735, t94748, t94749, t94756, t94758, t94761, t94762)
}
