//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 722/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk722<F: Float>(t438: F, t7614: F, t425: F, t7605: F, t1195: F, t2001: F, t431: F, t1966: F, t377: F) -> (F, F, F, F, F, F, F) {
    let t7622 = t7614 * t438;
    let t7624 = t7605 * t425;
    let t7625 = F::new(0.34299214494455789578e-2) * t7624;
    let t7626 = t2001 * t1195;
    let t7628 = t7605 * t431;
    let t7629 = F::new(0.17149607247227894789e-2) * t7628;
    let t7630 = t377 * t1966;
    (t7622, t7624, t7625, t7626, t7628, t7629, t7630)
}
