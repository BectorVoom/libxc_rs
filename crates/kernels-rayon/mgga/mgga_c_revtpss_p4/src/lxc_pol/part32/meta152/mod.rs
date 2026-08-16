//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk748;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta152(t1340: f64, t2496: f64, t1330: f64, t177: f64, t762: f64, t2626: f64, t1412: f64, t73: f64, t1389: f64, t1408: f64, t2736: f64, t1419: f64, t213: f64, t1425: f64, t560: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4037, t4038, t4039, t4042, t4049, t4062, t4064, t4071) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk748(t1340, t2496, t1330, t177, t762, t2626, t1412, t73, t1389, t1408, t2736, t1419, t213);
        let t4075 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk749(t1425, t560);
    (t4037, t4038, t4039, t4042, t4049, t4062, t4064, t4071, t4075)
}
