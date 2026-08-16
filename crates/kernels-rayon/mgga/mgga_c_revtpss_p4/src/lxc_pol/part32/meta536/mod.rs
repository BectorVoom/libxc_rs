//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1845;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta536(t1419: f64, t786: f64, t25877: f64, t2453: f64, t25949: f64, t25898: f64, t112: f64, t843: f64, t239: f64, t655: f64, t665: f64, t2339: f64, t624: f64, t10208: f64, t68: f64, t25081: f64, t7234: f64, t1923: f64, t26204: f64, t6977: f64, t1927: f64, t72: f64, t26205: f64, t6954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94890, t94913, t94921, t94973, t94975, t94976, t94978) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1845(t1419, t786, t25877, t2453, t25949, t25898, t112, t843, t239, t655, t665, t2339, t624);
        let (t94982, t95088, t95246, t95253, t95255) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1846(t10208, t68, t25081, t7234, t1923, t26204, t6977, t1927, t72, t843, t26205, t6954);
    (t94890, t94913, t94921, t94973, t94975, t94976, t94978, t94982, t95088, t95246, t95253, t95255)
}
