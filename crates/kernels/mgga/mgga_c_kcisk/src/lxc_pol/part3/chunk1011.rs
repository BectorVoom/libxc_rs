//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1011/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1011<F: Float>(t14909: F, t14910: F, t3952: F, t3973: F, t4402: F, t1580: F, t4384: F, t4397: F, t14875: F, t14878: F, t14886: F, t14893: F, t14898: F, t14902: F, t14906: F, t1583: F, t4381: F, t4388: F, t4393: F, t4403: F) -> F {
    let t14911 = t14909 * t14910;
    let t14912 = t3952 * t14911;
    let t14915 = t3973 * t4402;
    let t14916 = t1580 * t14915;
    let t14918 = t4397 * t4384;
    let t14920 = F::cast_from(0.16191709844559585492e0_f64) * t1580 * t14875 - F::cast_from(0.14392630972941853771e0_f64) * t14878 * t1583 - F::cast_from(0.71963154864709268853e-1_f64) * t4381 * t4388 - F::cast_from(0.95950873152945691806e-1_f64) * t4381 * t4393 + F::cast_from(0.2698618307426597582e-1_f64) * t14886 * t1583 - F::cast_from(0.53972366148531951639e-1_f64) * t4397 * t4403 - F::cast_from(0.2698618307426597582e-1_f64) * t1580 * t14893 - F::cast_from(0.2698618307426597582e-1_f64) * t1580 * t14898 - F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t14902 + F::cast_from(0.35981577432354634428e-1_f64) * t1580 * t14906 - F::cast_from(0.35981577432354634427e-1_f64) * t1580 * t14912 - F::cast_from(0.17990788716177317214e-1_f64) * t14916 + F::cast_from(0.17990788716177317214e-1_f64) * t14918;
    t14920
}
