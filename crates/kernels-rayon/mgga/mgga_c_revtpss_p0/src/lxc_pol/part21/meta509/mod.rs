//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2131;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2132;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta509(t11922: f64, t4906: f64, t3115: f64, t15957: f64, t4910: f64, t3117: f64, t3075: f64, t357: f64, t4781: f64, t11670: f64, t4890: f64, t3317: f64, t3299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16035, t16037, t16039, t16040, t16043, t16044, t16045, t16048) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2131(t11922, t4906, t3115, t15957, t4910, t3117, t3075, t357, t4781, t11670, t4890);
        let t16049 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2132(t16048, t3317);
        let t16052 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2133(t16048, t3299);
    (t16035, t16037, t16039, t16040, t16043, t16044, t16045, t16048, t16049, t16052)
}
