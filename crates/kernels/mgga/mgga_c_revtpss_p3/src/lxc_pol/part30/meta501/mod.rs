//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1868;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1869;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1870;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta501<F: Float>(t1248: F, t1287: F, t7653: F, t1294: F, t7638: F, t7652: F, t1243: F, t7627: F, t1032: F, t1269: F, t2148: F, t1203: F, t7637: F, t12626: F, t2147: F, t7635: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26924, t26928, t26931, t26933, t26936) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1868::<F>(t1248, t1287, t7653, t1294, t7638, t7652, t1243, t7627, t1032, t1269);
        let t26937 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1869::<F>(t2148, t26936);
        let (t26941, t26945, t26948) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1870::<F>(t1203, t7627, t7637, t1294, t7652, t12626, t2147);
        let t26949 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1871::<F>(t26948, t7635);
    (t26924, t26928, t26931, t26933, t26936, t26937, t26941, t26945, t26948, t26949)
}
