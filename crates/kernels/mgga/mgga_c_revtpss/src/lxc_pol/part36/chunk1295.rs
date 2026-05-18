//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1295/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1295<F: Float>(t20783: F, t26880: F, t5326: F, t8184: F, t20846: F, t26824: F, t29062: F, t5362: F, t1256: F, t30816: F, t30812: F, t1243: F, t30840: F) -> (F, F, F, F, F, F, F) {
    let t112468 = t26880 * t20783;
    let t112480 = t5326 * t8184;
    let t112483 = t26824 * t20846;
    let t112485 = t29062 * t5362;
    let t112487 = t30816 * t1256;
    let t112491 = t30812 * t1256;
    let t112686 = t1243 * t30840;
    (t112468, t112480, t112483, t112485, t112487, t112491, t112686)
}
