//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta893 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3081;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta893<F: Float>(t1058: F, t15859: F, t3201: F, t4794: F, t15866: F, t15888: F, t4798: F, t343: F, t44: F, t816: F, t11821: F, t65: F, t11144: F, t11970: F, t1660: F, t27527: F, t2852: F, t11150: F, t27531: F, t15817: F, t3173: F, t16158: F, t3188: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53298, t53300, t53302, t53308, t53317, t53320, t53321) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3081::<F>(t1058, t15859, t3201, t4794, t15866, t15888, t4798, t343, t44, t816, t11821, t65);
        let (t53322, t53326, t53328, t53332, t53353, t53359) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3082::<F>(t11144, t53321, t11970, t1660, t27527, t2852, t11150, t27531, t15817, t3173, t16158, t3188);
    (t53298, t53300, t53302, t53308, t53317, t53320, t53322, t53326, t53328, t53332, t53353, t53359)
}
