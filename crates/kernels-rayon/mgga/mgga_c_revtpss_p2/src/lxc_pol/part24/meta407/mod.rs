//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1347;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta407(t235: f64, t4503: f64, t2453: f64, t123: f64, t125: f64, t2452: f64, t40633: f64, t810: f64, t10759: f64, t2735: f64, t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t798: f64, t9726: f64, t10899: f64, t794: f64, t159: f64, t216: f64, t2475: f64, t251: f64, t40321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40799, t40810, t40834, t40846) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1347(t235, t4503, t2453, t123, t125, t2452, t40633, t810, t10759, t2735, t10293, t240);
        let (t40850, t40861, t40864, t40868, t40902) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1348(t243, t40846, t813, t816, t798, t9726, t10899, t794, t159, t216, t2475, t251, t40321);
    (t40799, t40810, t40834, t40846, t40850, t40861, t40864, t40868, t40902)
}
