//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2060;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta483(t15234: f64, t973: f64, t2962: f64, t4673: f64, t11452: f64, t1621: f64, t2944: f64, t4708: f64, t972: f64, t1634: f64, t3006: f64, t2988: f64, t4711: f64, t3014: f64, t4707: f64, t11450: f64, t11461: f64, t11466: f64, t11554: f64, t15100: f64, t15103: f64, t15104: f64, t2945: f64, t2968: f64, t2987: f64, t3012: f64, t4690: f64, t4712: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15235, t15238, t15241, t15242, t15249, t15252, t15255) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2060(t15234, t973, t2962, t4673, t11452, t1621, t2944, t4708, t972, t1634, t3006, t2988, t4711);
        let (t15258, t15259, t15262) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2061(t3014, t4707, t972, t11450, t11461, t11466, t11554, t15100, t15103, t15104, t15235, t15238, t15242, t15249, t15252, t15255, t2945, t2968, t2987, t3012, t4690, t4712, t965);
    (t15235, t15238, t15241, t15242, t15249, t15252, t15255, t15258, t15259, t15262)
}
