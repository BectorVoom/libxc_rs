//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1230;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta193(t4590: f64, t936: f64, t1610: f64, t2869: f64, t934: f64, t2874: f64, t1600: f64, t2880: f64, t918: f64, t2848: f64, t2884: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t916: f64, t2897: f64, t923: f64, t1606: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4592, t4594, t4595, t4597, t4598, t4599, t4606) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1230(t4590, t936, t1610, t2869, t934, t2874, t1600, t2880, t918, t2848, t2884, t4571, t4576, t4581, t4585);
        let (t4607, t4614, t4615, t4617, t4620) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1231(t4606, t916, t1600, t2897, t918, t923, t1606, t698);
    (t4592, t4594, t4595, t4597, t4598, t4599, t4606, t4607, t4614, t4615, t4617, t4620)
}
