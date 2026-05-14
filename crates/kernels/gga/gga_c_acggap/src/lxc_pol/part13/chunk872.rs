//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 872/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk872<F: Float>(t29984: F, t315: F, t2134: F, t1960: F, t3883: F, t119: F, t7877: F, t3912: F, t7976: F, t872: F, t3919: F, t7948: F, t3909: F, t323: F, t3035: F, t3923: F, t609: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32063 = t315 * t29984;
    let t32064 = t32063 * t2134;
    let t32066 = t1960 * t3883;
    let t32069 = t119 * t7877;
    let t32073 = 0.65854491829355115987e0 * t1960 * t3912;
    let t32080 = t7976 * t872;
    let t32082 = t7948 * t3919;
    let t32084 = t1960 * t3909;
    let t32087 = t315 * t7877 * t323;
    let t32091 = 0.39512695097613069591e1 * t3035 * t609 * t3923;
    (t32064, t32066, t32069, t32073, t32080, t32082, t32084, t32087, t32091)
}
