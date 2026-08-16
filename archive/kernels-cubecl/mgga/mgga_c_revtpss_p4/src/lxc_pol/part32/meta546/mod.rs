//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1860;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta546<F: Float>(t7493: F, t9292: F, t136: F, t137: F, t2097: F, t94386: F, t94391: F, t9646: F, t9648: F, t25875: F, t96186: F, t26230: F, t94633: F, t25899: F, t1358: F, t2439: F, t7506: F, t785: F, t26276: F, t9285: F, t25944: F, t2457: F, t7531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96218, t96220, t96221, t96222, t96230, t96236, t96245) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1860::<F>(t7493, t9292, t136, t137, t2097, t94386, t94391, t9646, t9648, t25875, t96186, t26230, t94633);
        let (t96246, t96253, t96255, t96257, t96259) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861::<F>(t25899, t96245, t1358, t2439, t7506, t785, t26276, t9285, t25944, t136, t2457, t7531);
    (t96218, t96220, t96221, t96222, t96230, t96236, t96245, t96246, t96253, t96255, t96257, t96259)
}
