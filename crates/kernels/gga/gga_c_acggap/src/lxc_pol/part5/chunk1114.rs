//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1114/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1114<F: Float>(t19942: F, t19966: F, t59: F, t40: F, t87: F, t85: F, t14930: F, t14935: F, t11721: F, t11708: F, t19451: F, t19452: F, t19453: F, t19454: F, t19455: F, t19456: F, t19914: F, t19915: F, t19916: F, t19917: F, t19918: F) -> (F, F, F, F, F, F) {
    let t19968 = (t19942 + t19966) * t59;
    let t19970 = t40 * t19968 * t87;
    let t19972 = F::cast_from(0.19751673498613801407e-1_f64) * t19968 * t85;
    let t19973 = F::new(4.0) * t14930;
    let t19974 = F::new(2.0) * t14935;
    let t19975 = F::cast_from(0.20779030926817756511e3_f64) * t11721;
    let t19976 = -t19451 + t19452 + t19453 + t19454 - t19455 + t19456 + t19914 - t19915 + t19916 + t19917 + t11708 - t19918 + t19970 + t19972 + t19973 + t19974 + t19975;
    (t19970, t19972, t19973, t19974, t19975, t19976)
}
