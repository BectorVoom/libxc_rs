//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta574(t1358: f64, t2439: f64, t785: f64, t8085: f64, t1364: f64, t28905: f64, t786: f64, t96187: f64, t97688: f64, t28791: f64, t689: f64, t25899: f64, t136: f64, t2457: f64, t8094: f64, t94589: f64, t26072: f64, t28845: f64, t28840: f64, t686: f64, t72: f64, t25895: f64, t2470: f64, t28779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102139, t102143, t102164, t102165, t102167) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1899(t1358, t2439, t785, t8085, t1364, t28905, t786, t96187, t97688, t28791, t689, t25899);
        let (t102204, t102205, t102213, t102215, t102217, t102218) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1900(t136, t2457, t8094, t94589, t26072, t28845, t28840, t686, t72, t25895, t2470, t28779);
    (t102139, t102143, t102164, t102165, t102167, t102204, t102205, t102213, t102215, t102217, t102218)
}
