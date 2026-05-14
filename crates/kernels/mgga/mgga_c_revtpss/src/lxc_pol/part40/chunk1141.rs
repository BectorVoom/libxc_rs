//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1141/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1141<F: Float>(t14540: F, t14572: F, t14953: F, t14976: F, t868: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F) -> (F, F, F, F) {
    let t14978 = t14540 + t14572 + t14953 + t14976;
    let t14979 = t868 * t14978;
    let t14982 = t4533 * t72;
    let t14983 = t14982 * t686;
    let t14985 = 0.19514881078765566038e-1 * t2465 * t14983;
    let t14986 = t1569 * t867;
    let t14987 = t786 * t14986;
    let t14989 = 0.19514881078765566038e-1 * t14987 * t2467;
    let t14990 = t4480 * t122;
    let t14991 = t14990 * t2466;
    (t14979, t14985, t14989, t14991)
}
