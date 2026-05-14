//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 961/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk961<F: Float>(t2576: F, t3579: F, t1007: F, t1014: F, t260: F, t2617: F, t3591: F, t8965: F, t8968: F, t8975: F, t8977: F, t8979: F, t8982: F, t8985: F, t8988: F, t8992: F, t8995: F, t8999: F, t9002: F, t9032: F, t9070: F, t9097: F, t9101: F, t9204: F, t9269: F) -> (F, F) {
    let t9273 = t2576 * t3579;
    let t9274 = t9273 * t1007;
    let t9279 = 0.10389515463408878255e3 * t1014 * t8965 - 0.35089341735807877242e1 * t1014 * t8968 - 0.5848223622634646207e0 * t3591 * t2617 + t8975 - t8977 + t8979 - t8982 - t8985 - t8988 + t8992 + t8995 + t8999 - 0.10254018858216406658e4 * t1014 * t9002 + t260 * (t9070 + t9097 + t9204 + t9269) + 0.23392894490538584828e1 * t1014 * t9274 + 0.19751673498613801407e-1 * t260 * t9032 + t9101;
    (t9274, t9279)
}
