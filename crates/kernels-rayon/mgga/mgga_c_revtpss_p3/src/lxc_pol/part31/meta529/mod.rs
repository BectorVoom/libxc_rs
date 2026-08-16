//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1905;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta529(t28076: f64, t72: f64, t1927: f64, t6977: f64, t7715: f64, t6973: f64, t7719: f64, t4237: f64, t76: f64, t1926: f64, t13269: f64, t38: f64, t1497: f64, t640: f64, t77: f64, t4241: f64, t84: f64, t1470: f64, t2242: f64, t1923: f64, t1928: f64, t25106: f64, t6954: f64, t6958: f64, t6974: f64, t6978: f64, t7702: f64, t7706: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28077, t28078, t28081, t28086, t28089, t28090, t28093) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1905(t28076, t72, t1927, t6977, t7715, t6973, t7719, t4237, t76, t1926, t13269, t38);
        let (t28105, t28109, t28112, t28115) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1906(t1497, t640, t77, t4241, t84, t1470, t2242, t1923, t1928, t25106, t28078, t28081, t28086, t28090, t28093, t6954, t6958, t6974, t6978, t7702, t7706, t7716, t7720);
    (t28077, t28078, t28081, t28086, t28089, t28090, t28093, t28105, t28109, t28112, t28115)
}
