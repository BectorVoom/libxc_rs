//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1715;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1716;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1717;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta378(t1041: f64, t16163: f64, t1651: f64, t3181: f64, t3168: f64, t4878: f64, t11150: f64, t11144: f64, t11852: f64, t3124: f64, t4820: f64, t1655: f64, t697: f64, t1011: f64, t372: f64, t4806: f64, t15688: f64, t3299: f64, t1678: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16165, t16170, t16190, t16199, t16208, t16218, t16219) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1715(t1041, t16163, t1651, t3181, t3168, t4878, t11150, t11144, t11852, t3124, t4820, t1655, t697);
        let (t16220, t16222) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1716(t1011, t16219, t372, t4806);
        let t16226 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1717(t15688, t3299);
        let t16284 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1718(t1678, t3057);
    (t16165, t16170, t16190, t16199, t16208, t16218, t16219, t16220, t16222, t16226, t16284)
}
