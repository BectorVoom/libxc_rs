//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta875 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3039;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta875(t14574: f64, t2439: f64, t2777: f64, t40297: f64, t4500: f64, t10069: f64, t14504: f64, t4423: f64, t860: f64, t1558: f64, t2760: f64, t14557: f64, t9303: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64, t10542: f64, t14520: f64, t2783: f64, t4469: f64, t786: f64, t2801: f64, t10073: f64, t14588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51355, t51371, t51373, t51375, t51380, t51390) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3039(t14574, t2439, t2777, t40297, t4500, t10069, t14504, t4423, t860, t1558, t2760, t14557, t9303);
        let (t51403, t51408, t51418, t51421, t51422, t51424) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3040(t4519, t9292, t2798, t4499, t9288, t10542, t14520, t2783, t4469, t786, t2801, t10073, t14588);
    (t51355, t51371, t51373, t51375, t51380, t51390, t51403, t51408, t51418, t51421, t51422, t51424)
}
