//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 803/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk803<F: Float>(t1347: F, t2001: F, t1352: F, t1998: F, t1446: F, t1423: F, t542: F, t7614: F, t537: F, t532: F, t7605: F, t1569: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8704 = t2001 * t1347;
    let t8706 = t1998 * t1352;
    let t8708 = t2001 * t1446;
    let t8710 = t1998 * t1423;
    let t8712 = t7614 * t542;
    let t8714 = t7614 * t537;
    let t8716 = t7614 * t532;
    let t8718 = t7605 * t532;
    let t8720 = t2001 * t1569;
    (t8704, t8706, t8708, t8710, t8712, t8714, t8716, t8718, t8720)
}
