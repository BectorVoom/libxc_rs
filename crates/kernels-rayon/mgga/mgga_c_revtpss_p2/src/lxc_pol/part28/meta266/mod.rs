//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1193;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1194;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1195;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta266(t30: f64, t1996: f64, t45: f64, t606: f64, t7099: f64, t7194: f64, t33: f64, t775: f64, t890: f64, t1113: f64, t1940: f64, t1963: f64, t2403: f64, t7087: f64, t7091: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t265: f64, t502: f64, t7193: f64, t2003: f64, t57: f64, rho1: f64, t1936: f64, t2322: f64, t5523: f64, t1312: f64, t7002: f64, t670: f64, t6983: f64, t6985: f64, t1315: f64, t196: f64, t197: f64, t2035: f64, t2033: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7199, t7200, t7207, t7214) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1193(t30, t1996, t45, t606, t7099, t7194, t33, t775, t890, t1113, t1940, t1963, t2403, t7087, t7091, dens_threshold, rho0, zeta_threshold);
        let (t7215, t7221) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1194(t33, t265, t502, t7193, t2003, t57, t606, t7214, t7199, dens_threshold, rho1, zeta_threshold);
        let (t7231, t7234, t7235) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1195(t1936, t2322, t5523, t1312, t7002, t670, t6983, t6985, t1315, t196, t197);
        let (t7236, t7237) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1196(t2035, t7235, t2033, t531);
    (t7200, t7207, t7215, t7221, t7231, t7234, t7235, t7236, t7237)
}
