//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 782/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk782<F: Float>(t2986: F, t9317: F, t2003: F, t3039: F, t144: F, t1736: F, t5526: F, t674: F, t8893: F, t5542: F, t5708: F, t5211: F, t5713: F) -> (F, F, F, F, F, F) {
    let t9318 = t2986 * t9317;
    let t9320 = t3039 * t2003;
    let t9323 = t1736 * t144;
    let t9325 = t9323 * t674 * t5526;
    let t9326 = t8893 * t9325;
    let t9328 = t5708 * t5542;
    let t9330 = t5211 * t144 * t5713;
    (t9318, t9320, t9325, t9326, t9328, t9330)
}
