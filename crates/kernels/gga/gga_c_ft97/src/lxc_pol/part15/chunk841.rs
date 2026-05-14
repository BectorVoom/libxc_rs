//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 841/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk841<F: Float>(t20685: F, t8392: F, t1882: F, t20888: F, t20725: F, t20706: F, t20880: F, t20927: F, t20912: F, t20894: F, t20945: F, t20729: F, t20733: F, t20711: F, t20909: F, t20972: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77325 = t8392 * t20685;
    let t77346 = t1882 * t20888;
    let t77383 = t1882 * t20725;
    let t77386 = t1882 * t20706;
    let t77411 = t1882 * t20880;
    let t77452 = t8392 * t20927;
    let t77481 = t1882 * t20912;
    let t77487 = t1882 * t20894;
    let t77489 = t1882 * t20945;
    let t77491 = t1882 * t20729;
    let t77505 = t1882 * t20733;
    let t77521 = t1882 * t20711;
    let t77575 = t1882 * t20909;
    let t77602 = t604 * t20972;
    (t77325, t77346, t77383, t77386, t77411, t77452, t77481, t77487, t77489, t77491, t77505, t77521, t77575, t77602)
}
