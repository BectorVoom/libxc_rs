//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1128/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1128<F: Float>(t1769: F, t8827: F, t5257: F, t8983: F, t6892: F, t8959: F, t9005: F, t16402: F, t3413: F, t3444: F, t5384: F, t496: F, t8775: F) -> (F, F, F, F, F, F, F) {
    let t24387 = t1769 * t8827;
    let t24402 = t5257 * t8983;
    let t24421 = t6892 * t8959;
    let t24461 = t5257 * t9005;
    let t24487 = t16402 * t3413;
    let t24489 = t5384 * t3444;
    let t24527 = t496 * t8775;
    (t24387, t24402, t24421, t24461, t24487, t24489, t24527)
}
