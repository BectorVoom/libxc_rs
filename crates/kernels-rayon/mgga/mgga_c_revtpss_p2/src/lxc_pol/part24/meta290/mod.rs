//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1071;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1072;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta290(t378: f64, t6235: f64, t1678: f64, t4746: f64, t6343: f64, t994: f64, t19462: f64, t6461: f64, t698: f64, t6464: f64, t6467: f64, t6422: f64, t689: f64, t6426: f64, t6430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20178, t20191, t20204, t20211, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1071(t378, t6235, t1678, t4746, t6343, t994, t19462, t6461, t698, t6464, t6467, t6422, t689);
        let t20285 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1072(t6426, t689);
        let t20287 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1073(t6430, t689);
    (t20178, t20191, t20204, t20211, t20276, t20278, t20280, t20283, t20285, t20287)
}
