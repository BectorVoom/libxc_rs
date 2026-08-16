//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1577;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta418(t15559: f64, t981: f64, t3336: f64, t5019: f64, t11108: f64, t1699: f64, t3022: f64, t4725: f64, t11465: f64, t1633: f64, t3015: f64, t3026: f64, t4719: f64, t1695: f64, t3075: f64, t1079: f64, t3215: f64, t4858: f64, t372: f64, t4872: f64, t4786: f64, t4873: f64, t11696: f64, t4781: f64, t3092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15561, t15562, t15566, t15571, t15575, t15577) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1577(t15559, t981, t3336, t5019, t11108, t1699, t3022, t4725, t11465, t1633, t3015, t3026, t4719);
        let (t15579, t15583, t15586, t15592) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1578(t1695, t3075, t1079, t3215, t4858, t372, t4872, t4786, t4873, t11696, t4781, t3092);
    (t15561, t15562, t15566, t15571, t15575, t15577, t15579, t15583, t15586, t15592)
}
