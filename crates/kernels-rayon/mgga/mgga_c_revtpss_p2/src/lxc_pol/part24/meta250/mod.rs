//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1013;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1014;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1015;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta250(t2435: f64, t4322: f64, t1596: f64, t2873: f64, t1614: f64, t2942: f64, t1606: f64, t2439: f64, t1593: f64, t1626: f64, t3011: f64, t2967: f64, t2986: f64, t2923: f64, t3090: f64, t4954: f64, t1646: f64, t3056: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15063, t15101, t15104, t15123, t15189) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1013(t2435, t4322, t1596, t2873, t1614, t2942, t1606, t2439, t1593);
        let (t15350, t15406, t15413, t15421, t15618) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1014(t1626, t3011, t1614, t2967, t2986, t1596, t2923, t3090, t4954);
        let t15669 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1015(t1646, t3056);
        let t15670 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1016(t15669, t225);
    (t15063, t15101, t15104, t15123, t15189, t15350, t15406, t15413, t15421, t15618, t15669, t15670)
}
