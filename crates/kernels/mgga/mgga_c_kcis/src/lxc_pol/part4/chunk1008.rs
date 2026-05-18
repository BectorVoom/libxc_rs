//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1008/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1008<F: Float>(t1484: F, t3251: F, t1098: F, t3811: F, t11402: F, t513: F, t1416: F, t3820: F, t1317: F, t3838: F, t11407: F, t3843: F) -> (F, F, F, F, F, F, F) {
    let t11723 = t3251 * t1484;
    let t11725 = t1098 * t3811;
    let t11727 = t11402 * t513;
    let t11730 = t3820 * t1416;
    let t11736 = t1317 * t3838;
    let t11746 = F::new(0.12841111111111111111e-1) * t11407;
    let t11767 = t1098 * t3843;
    (t11723, t11725, t11727, t11730, t11736, t11746, t11767)
}
