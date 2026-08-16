//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta478(t4135: f64, t4147: f64, t2034: f64, t2014: f64, t10416: f64, t1936: f64, t13435: f64, t2322: f64, t7002: f64, t13440: f64, t5523: f64, t112: f64, t239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25802, t25803, t25804, t25812, t25814, t25816, t25818, t25820, t25821) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1804(t4135, t4147, t2034, t2014, t10416, t1936, t13435, t2322, t7002, t13440, t5523, t112, t239);
    (t25802, t25803, t25804, t25812, t25814, t25816, t25818, t25820, t25821)
}
