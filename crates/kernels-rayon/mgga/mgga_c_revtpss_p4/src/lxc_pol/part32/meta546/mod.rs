//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1860;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta546(t7493: f64, t9292: f64, t136: f64, t137: f64, t2097: f64, t94386: f64, t94391: f64, t9646: f64, t9648: f64, t25875: f64, t96186: f64, t26230: f64, t94633: f64, t25899: f64, t1358: f64, t2439: f64, t7506: f64, t785: f64, t26276: f64, t9285: f64, t25944: f64, t2457: f64, t7531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96218, t96220, t96221, t96222, t96230, t96236, t96245) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1860(t7493, t9292, t136, t137, t2097, t94386, t94391, t9646, t9648, t25875, t96186, t26230, t94633);
        let (t96246, t96253, t96255, t96257, t96259) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861(t25899, t96245, t1358, t2439, t7506, t785, t26276, t9285, t25944, t136, t2457, t7531);
    (t96218, t96220, t96221, t96222, t96230, t96236, t96245, t96246, t96253, t96255, t96257, t96259)
}
