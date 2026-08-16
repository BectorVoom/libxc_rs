//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta945 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta945(t17550: f64, t372: f64, t17352: f64, t3153: f64, t3623: f64, t53667: f64, t45619: f64, t3666: f64, t5390: f64, t17794: f64, t1261: f64, t17203: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t58899, t58909, t58919, t58920, t58927, t58960, t58975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3182(t17550, t372, t17352, t3153, t3623, t53667, t45619, t3666, t5390, t17794, t1261, t17203, t3172);
    (t58899, t58909, t58919, t58920, t58927, t58960, t58975)
}
