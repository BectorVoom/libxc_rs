//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 736/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk736<F: Float>(t2079: F, t8689: F, t1451: F, t2001: F, t1434: F, t1998: F, t1441: F, t1429: F, t1418: F, t1347: F, t1352: F, t1446: F, t1423: F, t542: F, t7614: F, t537: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8690 = t2079 * t8689;
    let t8692 = t2001 * t1451;
    let t8694 = t1998 * t1434;
    let t8696 = t2001 * t1441;
    let t8698 = t2001 * t1429;
    let t8700 = t2001 * t1418;
    let t8704 = t2001 * t1347;
    let t8706 = t1998 * t1352;
    let t8708 = t2001 * t1446;
    let t8710 = t1998 * t1423;
    let t8712 = t7614 * t542;
    let t8714 = t7614 * t537;
    (t8690, t8692, t8694, t8696, t8698, t8700, t8704, t8706, t8708, t8710, t8712, t8714)
}
