//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 870/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk870<F: Float>(t32003: F, t32004: F, t7934: F, t309: F, t955: F, t7932: F, t7963: F, t609: F, t848: F, t464: F, t2122: F, t323: F, t851: F, t14575: F, t7942: F, t16020: F) -> (F, F, F, F, F, F, F) {
    let t32006 = t32003 * t32004 * t7934;
    let t32010 = t955 * t309;
    let t32012 = t7963 * t7932 * t32010;
    let t32029 = t848 * t609;
    let t32030 = t32029 * t464;
    let t32033 = t851 * t2122 * t323;
    let t32036 = t7942 * t7932 * t14575;
    let t32039 = t7942 * t7932 * t16020;
    (t32006, t32012, t32029, t32030, t32033, t32036, t32039)
}
