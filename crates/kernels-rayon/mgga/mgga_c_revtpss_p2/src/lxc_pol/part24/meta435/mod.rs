//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta435(t794: f64, t9747: f64, t2699: f64, t3943: f64, t136: f64, t9941: f64, t1386: f64, t820: f64, t9948: f64, t159: f64, t216: f64, t4010: f64, t2482: f64, t2668: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t10111: f64, t1408: f64, t9720: f64, t40735: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46691, t46694, t46716, t46722, t46730) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387(t794, t9747, t2699, t3943, t136, t9941, t1386, t820, t9948, t159, t216, t4010);
        let (t46740, t46760, t46766, t46784, t46800) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388(t1386, t2482, t2668, t1376, t40757, t2681, t4000, t820, t10111, t1408, t9720, t40735, t535);
    (t46691, t46694, t46716, t46722, t46730, t46740, t46760, t46766, t46784, t46800)
}
