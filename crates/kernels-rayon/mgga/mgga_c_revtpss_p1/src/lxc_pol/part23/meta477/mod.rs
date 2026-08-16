//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta477(t11860: f64, t19501: f64, t3117: f64, t19611: f64, t3095: f64, t3092: f64, t19414: f64, t247: f64, t3116: f64, t1651: f64, t4866: f64, t1045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20074, t20075, t20078, t20079, t20083, t20089, t20090) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1929(t11860, t19501, t3117, t19611, t3095, t3092, t19414, t247, t3116, t1651, t4866, t1045);
    (t20074, t20075, t20078, t20079, t20083, t20089, t20090)
}
