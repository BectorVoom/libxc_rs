//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2855/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2855<F: Float>(t3431: F, t408: F, t3434: F, t1126: F, t12247: F, t3800: F, t3140: F, t3552: F, t3599: F, t3362: F, t3603: F, t2251: F) -> (F, F, F, F, F, F, F, F) {
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = F::new(1.0) / t44092;
    let t44101 = t1126 * t12247;
    let t44125 = t3800 * t3800;
    let t44126 = F::new(1.0) / t44125;
    let t44169 = t3552 * t3140;
    let t44170 = t44169 * t3599;
    let t44190 = t3603 * t3362;
    let t44191 = t44190 * t2251;
    (t44091, t44093, t44101, t44126, t44169, t44170, t44190, t44191)
}
