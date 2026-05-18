//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 798/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk798<F: Float>(t1429: F, t2001: F, t1418: F, t1347: F, t1352: F, t1998: F, t1446: F, t1423: F, t542: F, t7614: F, t537: F, t532: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8698 = t2001 * t1429;
    let t8700 = t2001 * t1418;
    let t8704 = t2001 * t1347;
    let t8706 = t1998 * t1352;
    let t8708 = t2001 * t1446;
    let t8710 = t1998 * t1423;
    let t8712 = t7614 * t542;
    let t8714 = t7614 * t537;
    let t8716 = t7614 * t532;
    (t8698, t8700, t8704, t8706, t8708, t8710, t8712, t8714, t8716)
}
