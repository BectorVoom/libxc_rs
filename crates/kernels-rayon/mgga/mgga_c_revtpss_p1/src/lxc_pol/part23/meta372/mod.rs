//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1701;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta372(t1040: f64, t15816: f64, t1647: f64, t3140: f64, t3149: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64, t1062: f64, t4797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15817, t15822, t15823, t15827, t15829, t15830) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1701(t1040, t15816, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
        let t15850 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1702(t1062, t4797);
    (t15817, t15822, t15823, t15827, t15829, t15830, t15850)
}
