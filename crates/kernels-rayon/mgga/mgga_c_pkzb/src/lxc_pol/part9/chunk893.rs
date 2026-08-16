//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 893/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk893(t2393: f64, t937: f64, t133: f64, t6506: f64, t945: f64, t2387: f64, t410: f64, t6455: f64, t394: f64, t6012: f64, t6556: f64, t2436: f64, t2439: f64, t2443: f64, t2448: f64, t3270: f64, t397: f64, t6535: f64, t6555: f64, t6558: f64, t6561: f64, t6565: f64, t6566: f64, t6569: f64, t6571: f64, t6574: f64, t943: f64, t946: f64) -> (f64, f64, f64) {
    let t6579 = t2393 * t937;
    let t6582 = t6506 * t133;
    let t6583 = t6582 * t945;
    let t6586 = t410 * t2387;
    let t6590 = t6455 * t410;
    let t6591 = t6012 * t394;
    let t6592 = t6556 * t6591;
    let t6597 = 0.39512695097613069591e1_f64 * t6555 * t6558 + 0.39512695097613069591e1_f64 * t6561 * t2436 + 0.39512695097613069591e1_f64 * t6565 * t6566 - 0.39512695097613069591e1_f64 * t6569 * t6571 + 0.19756347548806534796e1_f64 * t6574 * t946 + 0.19756347548806534796e1_f64 * t2439 * t2443 - 0.19756347548806534796e1_f64 * t6579 * t2448 + 0.65854491829355115987e0_f64 * t943 * t6583 - 0.19756347548806534796e1_f64 * t2393 * t6586 * t3270 + 0.65854491829355115987e0_f64 * t6590 * t6592 + 0.65854491829355115987e0_f64 * t397 * t6535;
    (t6583, t6592, t6597)
}
