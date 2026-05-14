//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 907/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk907<F: Float>(t11467: F, t3011: F, t973: F, t981: F, t2986: F, t972: F, t3007: F, t11465: F, t3014: F, t11501: F, t964: F, t11572: F, t300: F, t11506: F, t11509: F, t11114: F, t11118: F, t11530: F, t11533: F, t11547: F, t11596: F) -> (F, F, F, F, F, F, F) {
    let t11598 = t3011 * t11467 * t973;
    let t11600 = 0.35089341735807877242e1 * t981 * t11598;
    let t11601 = t2986 * t972;
    let t11602 = t11601 * t3007;
    let t11604 = 0.35089341735807877242e1 * t981 * t11602;
    let t11606 = t11465 * t11467 * t3014;
    let t11608 = 0.10389515463408878255e3 * t981 * t11606;
    let t11610 = t964 * t11501 * t973;
    let t11612 = 0.5848223622634646207e0 * t981 * t11610;
    let t11614 = 0.19751673498613801407e-1 * t300 * t11572;
    let t11616 = t11506 * t11467 * t11509;
    let t11618 = 0.10254018858216406658e4 * t981 * t11616;
    let t11619 = t11596 - t11600 + t11604 + t11608 - t11612 + t11614 - t11547 - t11618 - t11530 + t11533 - t11114 + t11118;
    (t11600, t11604, t11608, t11612, t11614, t11618, t11619)
}
