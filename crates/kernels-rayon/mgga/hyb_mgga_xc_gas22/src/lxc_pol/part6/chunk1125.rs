//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1125/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1125(t4300: f64, t986: f64, t4297: f64, t2562: f64, t4296: f64, t3546: f64, t3550: f64, t1007: f64, t11076: f64, t11079: f64, t11086: f64, t11090: f64, t11095: f64, t11098: f64, t1422: f64, t2538: f64, t2560: f64, t2599: f64, t3527: f64, t3547: f64, t3551: f64, t6993: f64, t7002: f64, t9205: f64, t9210: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11101 = t4300 * t986;
    let t11104 = t4297 * t986;
    let t11107 = t4296 * t2562;
    let t11108 = t11107 * t986;
    let t11111 = t3550 * t3546;
    let t11114 = 0.5848223622634646207e0_f64 * t11076 * t1007 + 1.0_f64 * t11079 * t988 + 2.0_f64 * t9205 * t1422 + 2.0_f64 * t3527 * t3547 + 0.34631718211362927518e2_f64 * t2599 * t11086 + 0.10254018858216406658e4_f64 * t6993 * t11090 + 0.64327917994770140268e2_f64 * t9210 * t3551 + 6.0_f64 * t2560 * t11095 - 4.0_f64 * t2538 * t11098 - 0.19298375398431042081e3_f64 * t7002 * t11101 - 2.0_f64 * t2538 * t11104 + 0.32163958997385070134e2_f64 * t2560 * t11108 + 0.64327917994770140268e2_f64 * t2560 * t11111;
    (t11101, t11104, t11107, t11108, t11111, t11114)
}
