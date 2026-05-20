//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1443;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1444;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta401<F: Float>(t11150: F, t1469: F, t2251: F, t2850: F, t128: F, t4573: F, t904: F, t2908: F, t141: F, t930: F, t4625: F, t698: F, t4622: F, t15130: F, t15137: F, t15142: F, t15147: F, t15151: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15154, t15156) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1443::<F>(t11150, t1469, t2251, t2850, t128);
        let (t15158, t15160) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1444::<F>(t2251, t4573, t904, t128);
        let (t15163, t15166, t15168, t15170, t15173, t15175) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1445::<F>(t15154, t2908, t141, t15158, t930, t4625, t698, t4622, t15130, t15137, t15142, t15147, t15151, t15156, t15160);
    (t15154, t15156, t15158, t15160, t15163, t15166, t15168, t15170, t15173, t15175)
}
