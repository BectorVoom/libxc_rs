//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk982;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk983;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta208(t10276: f64, t22: f64, t2224: f64, t588: f64, t27: f64, t584: f64, t20: f64, t596: f64, t12: f64, t583: f64, t2231: f64, t2237: f64, t592: f64, t2236: f64, t3: f64, t25: f64, t10271: f64, t10273: f64, t10275: f64, t2240: f64, t602: f64, t2246: f64, t599: f64, t88: f64, t89: f64, t90: f64, t29: f64, t2248: f64, t644: f64, t2315: f64, t606: f64, t70: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10278, t10280, t10282, t10284, t10285, t10287, t10289, t10290) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk982(t10276, t22, t2224, t588, t27, t584, t20, t596, t12, t583, t2231, t2237, t592);
        let (t10292, t10293, t10296) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk983(t10290, t2236, t3, t25, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289);
        let (t10298, t10301, t10308, t10309, t10310, t10313, t10317) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk984(t2240, t602, t2246, t599, t88, t89, t90, t29, t2248, t644, t2315, t606, t70, t72);
    (t10285, t10292, t10293, t10296, t10298, t10301, t10308, t10309, t10310, t10313, t10317)
}
