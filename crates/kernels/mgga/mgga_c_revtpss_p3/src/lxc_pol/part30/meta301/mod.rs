//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1284;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta301<F: Float>(t216: F, t9747: F, t3989: F, t4014: F, t221: F, t3889: F, t3979: F, t3978: F, t1408: F, t2482: F, t596: F, t3981: F, t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9748, t9753, t9761, t9762, t9765, t9766) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1284::<F>(t216, t9747, t3989, t4014, t221, t3889, t3979, t3978, t1408, t2482, t596, t3981);
        let (t9768, t9769, t9771, t9775) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1285::<F>(t3923, t550, t543, t3992, t2661, t212, t225, t596, t816);
    (t9748, t9753, t9761, t9762, t9765, t9766, t9768, t9769, t9771, t9775)
}
