//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta599(t1269: f64, t7642: f64, t8945: f64, t1243: f64, t26884: f64, t12941: f64, t7618: f64, t13068: f64, t7617: f64, t26873: f64, t3704: f64, t12959: f64, t26880: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t97082, t97095, t97112, t97120, t97125, t97136) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2060(t1269, t7642, t8945, t1243, t26884, t12941, t7618, t13068, t7617, t26873, t3704, t12959, t26880);
    (t97082, t97095, t97112, t97120, t97125, t97136)
}
