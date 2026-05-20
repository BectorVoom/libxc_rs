//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1875;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1876;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta503<F: Float>(t1203: F, t1208: F, t487: F, t2142: F, t3790: F, t7652: F, t2148: F, t3727: F, t3566: F, t7635: F, t1214: F, t7638: F, t7637: F, t1209: F, t7627: F, t2150: F, t26884: F, t473: F, t460: F, t3555: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26982, t26983, t26984, t26988, t26991, t26994) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1875::<F>(t1203, t1208, t487, t2142, t3790, t7652, t2148, t3727, t3566, t7635);
        let (t26996, t26999) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1876::<F>(t1214, t7638, t7637, t1209, t7627);
        let (t27005, t27008, t27011) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1877::<F>(t2150, t26884, t473, t460, t7627, t2142, t3555);
    (t26982, t26983, t26984, t26988, t26991, t26994, t26996, t26999, t27005, t27008, t27011)
}
