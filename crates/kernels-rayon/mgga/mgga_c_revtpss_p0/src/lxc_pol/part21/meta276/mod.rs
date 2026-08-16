//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1501;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1502;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta276(t10039: f64, t869: f64, t689: f64, t2777: f64, t4092: f64, t2439: f64, t1419: f64, t3999: f64, t3923: f64, t555: f64, t4003: f64, t5744: f64, t2782: f64, t4086: f64, t543: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10040, t10041, t10043, t10044, t10049, t10059, t10061) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1501(t10039, t869, t689, t2777, t4092, t2439, t1419, t3999, t3923, t555, t4003, t5744);
        let (t10062, t10065, t10066, t10069) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1502(t10061, t2782, t10059, t4086, t543, t123, t212, t2434);
        let (t10070, t10073) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1503(t10069, t4089, t138, t2438, t785);
    (t10040, t10041, t10043, t10044, t10049, t10061, t10062, t10065, t10066, t10069, t10070, t10073)
}
