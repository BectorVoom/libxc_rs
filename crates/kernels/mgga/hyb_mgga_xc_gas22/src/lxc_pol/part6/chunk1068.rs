//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1068/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1068<F: Float>(t11107: F, t986: F, t3546: F, t3550: F, t1007: F, t11076: F, t11079: F, t11086: F, t11090: F, t11095: F, t11098: F, t11101: F, t11104: F, t1422: F, t2538: F, t2560: F, t2599: F, t3527: F, t3547: F, t3551: F, t6993: F, t7002: F, t9205: F, t9210: F, t988: F) -> (F, F, F) {
    let t11108 = t11107 * t986;
    let t11111 = t3550 * t3546;
    let t11114 = 0.5848223622634646207e0 * t11076 * t1007 + 1.0 * t11079 * t988 + 2.0 * t9205 * t1422 + 2.0 * t3527 * t3547 + 0.34631718211362927518e2 * t2599 * t11086 + 0.10254018858216406658e4 * t6993 * t11090 + 0.64327917994770140268e2 * t9210 * t3551 + 6.0 * t2560 * t11095 - 4.0 * t2538 * t11098 - 0.19298375398431042081e3 * t7002 * t11101 - 2.0 * t2538 * t11104 + 0.32163958997385070134e2 * t2560 * t11108 + 0.64327917994770140268e2 * t2560 * t11111;
    (t11108, t11111, t11114)
}
