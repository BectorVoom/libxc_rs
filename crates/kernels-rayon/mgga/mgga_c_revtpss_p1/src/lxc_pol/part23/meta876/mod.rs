//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta876 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2780;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta876(t22026: f64, t46802: f64, t9794: f64, t46694: f64, t6850: f64, t22294: f64, t48823: f64, t9816: f64, t1398: f64, t6843: f64, t22245: f64, t808: f64, t9736: f64, t22236: f64, t6884: f64, t9741: f64, t14104: f64, t47856: f64, t13729: f64, t2782: f64, t556: f64, t5774: f64, t2439: f64, t3895: f64, t6896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74677, t74682, t74698, t74700, t74711) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2780(t22026, t46802, t9794, t46694, t6850, t22294, t48823, t9816, t1398, t6843, t22245, t808, t9736);
        let (t74714, t74717, t74733, t74744, t74757) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781(t22236, t808, t9736, t6884, t9741, t14104, t47856, t13729, t2782, t556, t5774, t2439, t3895, t6896);
    (t74677, t74682, t74698, t74700, t74711, t74714, t74717, t74733, t74744, t74757)
}
