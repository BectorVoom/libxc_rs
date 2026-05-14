//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1030/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1030<F: Float>(t31142: F, t9727: F, t2060: F, t361: F, t9733: F, t7450: F, t9659: F, t13287: F, t31195: F, t38861: F, t13364: F, t38850: F, t1988: F, t9687: F, t2001: F, t6361: F) -> (F, F, F, F, F, F, F) {
    let t40083 = t31142 * t9727;
    let t40086 = t2060 * t361 * t9733;
    let t40089 = t7450 * t361 * t9659;
    let t40092 = t31195 * t13287 * t38861;
    let t40095 = t31195 * t13364 * t38850;
    let t40099 = t1988 * t9687;
    let t40101 = t2001 * t6361;
    (t40083, t40086, t40089, t40092, t40095, t40099, t40101)
}
