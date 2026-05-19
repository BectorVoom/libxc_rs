//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 893/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk893<F: Float>(t2393: F, t937: F, t133: F, t6506: F, t945: F, t2387: F, t410: F, t6455: F, t394: F, t6012: F, t6556: F, t2436: F, t2439: F, t2443: F, t2448: F, t3270: F, t397: F, t6535: F, t6555: F, t6558: F, t6561: F, t6565: F, t6566: F, t6569: F, t6571: F, t6574: F, t943: F, t946: F) -> (F, F, F) {
    let t6579 = t2393 * t937;
    let t6582 = t6506 * t133;
    let t6583 = t6582 * t945;
    let t6586 = t410 * t2387;
    let t6590 = t6455 * t410;
    let t6591 = t6012 * t394;
    let t6592 = t6556 * t6591;
    let t6597 = F::cast_from(0.39512695097613069591e1_f64) * t6555 * t6558 + F::cast_from(0.39512695097613069591e1_f64) * t6561 * t2436 + F::cast_from(0.39512695097613069591e1_f64) * t6565 * t6566 - F::cast_from(0.39512695097613069591e1_f64) * t6569 * t6571 + F::cast_from(0.19756347548806534796e1_f64) * t6574 * t946 + F::cast_from(0.19756347548806534796e1_f64) * t2439 * t2443 - F::cast_from(0.19756347548806534796e1_f64) * t6579 * t2448 + F::cast_from(0.65854491829355115987e0_f64) * t943 * t6583 - F::cast_from(0.19756347548806534796e1_f64) * t2393 * t6586 * t3270 + F::cast_from(0.65854491829355115987e0_f64) * t6590 * t6592 + F::cast_from(0.65854491829355115987e0_f64) * t397 * t6535;
    (t6583, t6592, t6597)
}
