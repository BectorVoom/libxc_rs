//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 818/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk818<F: Float>(t1736: F, t3292: F, t311: F, t314: F, t329: F, t6: F, t103: F, t3278: F, t962: F, t191: F, t2153: F, t1093: F) -> (F, F, F, F, F, F) {
    let t9551 = t3292 * t1736;
    let t9552 = t311 * t9551;
    let t9554 = t6 * t329 * t314;
    let t9555 = t103 * t9554;
    let t9556 = t9552 * t9555;
    let t9558 = t3278 * t962;
    let t9560 = t2153 * t191;
    let t9561 = t9560 * t1093;
    (t9552, t9554, t9555, t9556, t9558, t9561)
}
