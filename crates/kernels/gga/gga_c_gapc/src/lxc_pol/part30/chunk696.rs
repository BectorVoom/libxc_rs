//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 696/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk696<F: Float>(t4979: F, t8907: F, t1631: F, t190: F, t3707: F, t1743: F, t3113: F, t3112: F, t3117: F, t3123: F, t8798: F, t611: F, t8769: F, t5409: F, t204: F, t474: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8908 = t8907 * t4979;
    let t8910 = t1631 * t190;
    let t8911 = t8910 * t3707;
    let t8912 = t1743 * t8911;
    let t8913 = t8912 * t4979;
    let t8915 = t3113 * t3707;
    let t8916 = t3112 * t8915;
    let t8917 = t8916 * t3117;
    let t8919 = t8798 * t3123;
    let t8921 = t611 * t8769;
    let t8922 = t8921 * t5409;
    let t8926 = t474 * t204;
    (t8908, t8910, t8911, t8913, t8915, t8916, t8917, t8919, t8922, t8926)
}
