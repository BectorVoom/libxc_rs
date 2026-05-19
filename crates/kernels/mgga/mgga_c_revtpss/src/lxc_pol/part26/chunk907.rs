//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 907/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk907<F: Float>(t3022: F, t3026: F, t11467: F, t3011: F, t973: F, t981: F, t2986: F, t972: F, t3007: F, t11465: F, t3014: F, t11501: F, t964: F) -> (F, F, F, F, F) {
    let t11596 = F::cast_from(0.35089341735807877242e1_f64) * t3022 * t3026;
    let t11598 = t3011 * t11467 * t973;
    let t11600 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t11598;
    let t11601 = t2986 * t972;
    let t11602 = t11601 * t3007;
    let t11604 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t11602;
    let t11606 = t11465 * t11467 * t3014;
    let t11608 = F::cast_from(0.10389515463408878255e3_f64) * t981 * t11606;
    let t11610 = t964 * t11501 * t973;
    (t11596, t11600, t11604, t11608, t11610)
}
