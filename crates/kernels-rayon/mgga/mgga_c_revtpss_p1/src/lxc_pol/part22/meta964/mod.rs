//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta964 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3226;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta964(t14383: f64, t4311: f64, t40092: f64, t40094: f64, t50047: f64, t14386: f64, t4305: f64, t1544: f64, t2832: f64, t157: f64, t2251: f64, t6002: f64, t15071: f64, t1940: f64, t2403: f64, t40084: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t4556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t61197, t61198, t61199, t61200, t61202, t61203, t61209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3226(t14383, t4311, t40092, t40094, t50047, t14386, t4305, t1544, t2832, t157, t2251, t6002);
        let t61210 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227(t15071, t1940, t2403, t40084, t40088, t40099, t40103, t40115, t4556, t61197, t61198, t61199, t61200, t61202, t61203, t61209);
    (t61197, t61198, t61199, t61200, t61202, t61209, t61210)
}
