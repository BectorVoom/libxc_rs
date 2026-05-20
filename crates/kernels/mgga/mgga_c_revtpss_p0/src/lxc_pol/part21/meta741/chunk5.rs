//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2612/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2612<F: Float>(t10014: F, t14242: F, t10073: F, t14225: F, t1892: F, t5744: F, t786: F, t10026: F, t136: F, t2457: F, t3964: F, t5710: F) -> (F, F, F, F) {
    let t48079 = t10014 * t14242;
    let t48080 = F::cast_from(0.39029762157531132076e-1_f64) * t48079;
    let t48081 = t10073 * t14225;
    let t48082 = F::cast_from(0.19514881078765566038e-2_f64) * t48081;
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    let t48085 = t48084 * t10026;
    let t48089 = t3964 * t5710 * t136 * t2457;
    (t48080, t48082, t48085, t48089)
}
