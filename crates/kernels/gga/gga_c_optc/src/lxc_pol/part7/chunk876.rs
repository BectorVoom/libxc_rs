//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 876/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk876<F: Float>(t8914: F, t8915: F, t935: F, t450: F, t3101: F, t8912: F, t3107: F, t1128: F, t3128: F, t1121: F, t3245: F, t8493: F, t4289: F, t8498: F, t1114: F, t6554: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8916 = t8914 * t8915;
    let t8917 = t8916 * t935;
    let t8918 = t450 * t8917;
    let t8921 = t3101 * t8912;
    let t8922 = t8914 * t3107;
    let t8923 = t8922 * t935;
    let t8924 = t450 * t8923;
    let t8927 = t1128 * t3128;
    let t8928 = t1121 * t8927;
    let t8930 = t3245 * t8493;
    let t8933 = t4289 * t8498;
    let t8936 = t1114 * t6554;
    (t8917, t8918, t8921, t8923, t8924, t8928, t8930, t8933, t8936)
}
