//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1132;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta334(t11044: f64, t4481: f64, t2435: f64, t4477: f64, t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2444: f64, t689: f64, t213: f64, t4469: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t4321: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14995, t14998, t15004, t15006, t15008) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1132(t11044, t4481, t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444);
        let (t15010, t15011, t15015, t15018, t15045) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1133(t15008, t689, t213, t4469, t1580, t2440, t2439, t1569, t2453, t2458, t4321, t887);
    (t14995, t14998, t15004, t15006, t15010, t15011, t15015, t15018, t15045)
}
