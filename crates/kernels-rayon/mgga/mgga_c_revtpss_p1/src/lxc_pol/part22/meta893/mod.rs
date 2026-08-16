//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta893 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3081;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta893(t1058: f64, t15859: f64, t3201: f64, t4794: f64, t15866: f64, t15888: f64, t4798: f64, t343: f64, t44: f64, t816: f64, t11821: f64, t65: f64, t11144: f64, t11970: f64, t1660: f64, t27527: f64, t2852: f64, t11150: f64, t27531: f64, t15817: f64, t3173: f64, t16158: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53298, t53300, t53302, t53308, t53317, t53320, t53321) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3081(t1058, t15859, t3201, t4794, t15866, t15888, t4798, t343, t44, t816, t11821, t65);
        let (t53322, t53326, t53328, t53332, t53353, t53359) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3082(t11144, t53321, t11970, t1660, t27527, t2852, t11150, t27531, t15817, t3173, t16158, t3188);
    (t53298, t53300, t53302, t53308, t53317, t53320, t53322, t53326, t53328, t53332, t53353, t53359)
}
