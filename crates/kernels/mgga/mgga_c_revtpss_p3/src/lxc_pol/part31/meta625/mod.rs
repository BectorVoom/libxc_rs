//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2076;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta625<F: Float>(t7058: F, t99201: F, t25375: F, t99349: F, t14983: F, t25399: F, t7064: F, t99321: F, t25411: F, t99389: F, t2435: F, t7774: F, t25431: F, t14481: F, t1950: F, t2782: F, t2439: F, t7759: F, t780: F, t785: F, t2411: F, t27363: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t99481, t99485, t99487, t99491, t99493, t99495) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2076::<F>(t7058, t99201, t25375, t99349, t14983, t25399, t7064, t99321, t25411, t99389, t2435, t7774);
        let (t99496, t99502, t99520, t99522, t99555) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2077::<F>(t25431, t99495, t14481, t1950, t2782, t2439, t7759, t780, t785, t25411, t2411, t27363);
    (t99481, t99485, t99487, t99491, t99493, t99496, t99502, t99520, t99522, t99555)
}
