//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta545(t26072: f64, t26292: f64, t7493: f64, t9292: f64, t136: f64, t137: f64, t2097: f64, t94386: f64, t94391: f64, t1358: f64, t212: f64, t26333: f64, t689: f64, t9646: f64, t9648: f64, t7515: f64, t94894: f64, t25899: f64, t96192: f64, t25875: f64, t96186: f64, t94398: f64, t3916: f64, t96191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96211, t96218, t96220, t96221, t96222, t96226) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1881(t26072, t26292, t7493, t9292, t136, t137, t2097, t94386, t94391, t1358, t212, t26333, t689);
        let (t96230, t96232, t96234, t96236, t96237, t96239) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1882(t2097, t9646, t9648, t7515, t94894, t25899, t96192, t25875, t96186, t94398, t3916, t96191);
    (t96211, t96218, t96220, t96221, t96222, t96226, t96230, t96232, t96234, t96236, t96237, t96239)
}
