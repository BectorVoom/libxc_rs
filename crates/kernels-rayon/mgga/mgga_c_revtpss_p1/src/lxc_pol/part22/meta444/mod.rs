//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta444(t2435: f64, t4477: f64, t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2444: f64, t689: f64, t213: f64, t4469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14998, t15002, t15003, t15004, t15006, t15008, t15010, t15011) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2087(t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469);
    (t14998, t15002, t15003, t15004, t15006, t15008, t15010, t15011)
}
