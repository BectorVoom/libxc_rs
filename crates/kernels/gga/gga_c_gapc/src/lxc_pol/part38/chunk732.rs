//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 732/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk732<F: Float>(t144: F, t1736: F, t5526: F, t674: F, t8893: F, t5542: F, t5708: F, t5211: F, t5713: F, t2993: F, t3127: F, t5392: F, t3132: F, t5395: F, t3128: F, t5633: F) -> (F, F, F, F, F, F, F) {
    let t9323 = t1736 * t144;
    let t9325 = t9323 * t674 * t5526;
    let t9326 = t8893 * t9325;
    let t9328 = t5708 * t5542;
    let t9330 = t5211 * t144 * t5713;
    let t9331 = t9328 * t9330;
    let t9333 = t2993 * t3127;
    let t9334 = t9333 * t5392;
    let t9336 = t5395 * t3132;
    let t9337 = t9336 * t5392;
    let t9339 = t3128 * t5633;
    (t9325, t9326, t9330, t9331, t9334, t9337, t9339)
}
