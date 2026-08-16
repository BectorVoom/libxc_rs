//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 997/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk997(t2576: f64, t3579: f64, t1007: f64, t1014: f64, t260: f64, t2617: f64, t3591: f64, t8965: f64, t8968: f64, t8975: f64, t8977: f64, t8979: f64, t8982: f64, t8985: f64, t8988: f64, t8992: f64, t8995: f64, t8999: f64, t9002: f64, t9032: f64, t9070: f64, t9097: f64, t9101: f64, t9204: f64, t9269: f64) -> (f64, f64) {
    let t9273 = t2576 * t3579;
    let t9274 = t9273 * t1007;
    let t9279 = 0.10389515463408878255e3_f64 * t1014 * t8965 - 0.35089341735807877242e1_f64 * t1014 * t8968 - 0.5848223622634646207e0_f64 * t3591 * t2617 + t8975 - t8977 + t8979 - t8982 - t8985 - t8988 + t8992 + t8995 + t8999 - 0.10254018858216406658e4_f64 * t1014 * t9002 + t260 * (t9070 + t9097 + t9204 + t9269) + 0.23392894490538584828e1_f64 * t1014 * t9274 + 0.19751673498613801407e-1_f64 * t260 * t9032 + t9101;
    (t9274, t9279)
}
