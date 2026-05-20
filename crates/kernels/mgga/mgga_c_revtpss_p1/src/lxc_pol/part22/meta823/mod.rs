//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2939;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta823<F: Float>(t14220: F, t46495: F, t4086: F, t5710: F, t786: F, t4104: F, t1437: F, t2482: F, t5658: F, t2782: F, t48015: F, t543: F, t1882: F, t3923: F, t4003: F, t10022: F, t10014: F, t14242: F, t10073: F, t14225: F, t1892: F, t5744: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48041, t48048, t48049, t48058, t48066) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2939::<F>(t14220, t46495, t4086, t5710, t786, t4104, t1437, t2482, t5658, t2782, t48015, t543);
        let (t48073, t48076, t48079, t48081, t48083, t48084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940::<F>(t1882, t3923, t4003, t10022, t2782, t10014, t14242, t10073, t14225, t1892, t5744, t786);
    (t48041, t48048, t48049, t48058, t48066, t48073, t48076, t48079, t48081, t48083, t48084)
}
