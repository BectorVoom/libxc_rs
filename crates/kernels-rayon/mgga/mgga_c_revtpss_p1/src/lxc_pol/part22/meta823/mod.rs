//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2939;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta823(t14220: f64, t46495: f64, t4086: f64, t5710: f64, t786: f64, t4104: f64, t1437: f64, t2482: f64, t5658: f64, t2782: f64, t48015: f64, t543: f64, t1882: f64, t3923: f64, t4003: f64, t10022: f64, t10014: f64, t14242: f64, t10073: f64, t14225: f64, t1892: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48041, t48048, t48049, t48058, t48066) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2939(t14220, t46495, t4086, t5710, t786, t4104, t1437, t2482, t5658, t2782, t48015, t543);
        let (t48073, t48076, t48079, t48081, t48083, t48084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940(t1882, t3923, t4003, t10022, t2782, t10014, t14242, t10073, t14225, t1892, t5744, t786);
    (t48041, t48048, t48049, t48058, t48066, t48073, t48076, t48079, t48081, t48083, t48084)
}
