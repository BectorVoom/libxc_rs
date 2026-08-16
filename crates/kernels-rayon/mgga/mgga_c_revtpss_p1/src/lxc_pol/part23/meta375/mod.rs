//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1707;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta375(t15987: f64, t4579: f64, t1011: f64, t140: f64, t3252: f64, t4574: f64, t1012: f64, t11821: f64, t11922: f64, t4906: f64, t3115: f64, t11670: f64, t4890: f64, t3317: f64, t3299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15990, t15993) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1707(t15987, t4579, t1011, t140, t3252);
        let (t15996, t16012, t16035, t16037, t16048, t16049, t16052) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1708(t15993, t4574, t1011, t1012, t11821, t11922, t4906, t3115, t11670, t4890, t3317, t3299);
    (t15990, t15993, t15996, t16012, t16035, t16037, t16048, t16049, t16052)
}
