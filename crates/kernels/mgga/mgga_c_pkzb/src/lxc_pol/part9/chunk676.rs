//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 676/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk676<F: Float>(t3186: F, t3207: F, t406: F, t1235: F, t754: F, t46: F, t915: F) -> (F, F, F, F) {
    let t3208 = t3186 * t3207;
    let t3209 = t406 * t3208;
    let t3212 = t1235 * t754;
    let t3213 = t3212 * t46;
    let t3214 = t915 * t3213;
    (t3208, t3209, t3212, t3214)
}
