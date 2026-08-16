//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1159;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1160;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta263(t30: f64, t890: f64, t1940: f64, t1963: f64, t2403: f64, t605: f64, t7010: f64, t7087: f64, t7091: f64, t207: f64, t7086: f64, t198: f64, t775: f64, t892: f64, t33: f64, t1113: f64, t1936: f64, t2322: f64, t5523: f64, t1312: f64, t7002: f64, t1315: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7092, t7099, t7188, t7193) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1159(t30, t890, t1940, t1963, t2403, t605, t7010, t7087, t7091, t207, t7086, t198, t775, t892);
        let (t7200, t7207, t7214, t7226, t7228) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1160(t33, t775, t890, t1113, t1940, t1963, t2403, t7087, t7091, t1936, t2322, t5523);
        let (t7230, t7234, t7235) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1161(t1312, t7002, t1315, t196, t197);
    (t7092, t7099, t7188, t7193, t7200, t7207, t7214, t7226, t7228, t7230, t7234, t7235)
}
