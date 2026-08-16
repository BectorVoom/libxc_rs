//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2548;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta630(t1063: f64, t20054: f64, t19572: f64, t4894: f64, t3117: f64, t4900: f64, t11774: f64, t15926: f64, t20040: f64, t20046: f64, t20051: f64, t3106: f64, t3188: f64, t4892: f64, t4899: f64, t4912: f64, t6323: f64, t6327: f64, t6331: f64, t11860: f64, t19501: f64, t19611: f64, t3095: f64, t3092: f64, t19414: f64, t247: f64, t3116: f64, t1651: f64, t4866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20065, t20066, t20069, t20070, t20073) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2548(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
        let (t20074, t20075, t20078, t20079, t20083, t20089) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2549(t11860, t19501, t3117, t19611, t3095, t3092, t19414, t247, t3116, t1651, t4866);
    (t20065, t20066, t20069, t20070, t20073, t20074, t20075, t20078, t20079, t20083, t20089)
}
