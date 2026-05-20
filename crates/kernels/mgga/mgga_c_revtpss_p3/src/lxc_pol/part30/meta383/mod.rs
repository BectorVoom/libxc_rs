//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1437;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta383<F: Float>(t550: F, t5658: F, t543: F, t3992: F, t2661: F, t5610: F, t9775: F, t1889: F, t9779: F, t828: F, t9954: F, t1398: F, t1868: F, t3938: F, t3935: F, t1882: F, t4003: F, t1353: F, t3957: F, t5690: F, t1873: F, t9741: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13775, t13778, t13779, t13781, t13783, t13784) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1437::<F>(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954, t1398, t1868);
        let (t13786, t13789, t13790, t13793, t13797, t13798) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1438::<F>(t13784, t3938, t13783, t3935, t828, t1882, t4003, t1353, t1398, t3957, t5690, t1873, t9741);
    (t13775, t13778, t13779, t13781, t13786, t13789, t13790, t13793, t13797, t13798)
}
