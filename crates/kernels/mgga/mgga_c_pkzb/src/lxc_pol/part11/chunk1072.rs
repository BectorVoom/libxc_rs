//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1072/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1072<F: Float>(t10647: F, t16416: F, t10634: F, t5381: F, t10630: F, t1727: F, t2639: F, t8914: F, t10586: F, t5257: F, t10582: F, t10627: F, t1721: F, t600: F, t10578: F, t6892: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28979 = t16416 * t10647;
    let t28990 = t5381 * t10634;
    let t28992 = t1727 * t10630;
    let t28995 = t8914 * t2639;
    let t28999 = t5257 * t10586;
    let t29001 = t5257 * t10582;
    let t29003 = t10627 * t1721;
    let t29004 = t29003 * t600;
    let t29008 = t6892 * t10578;
    (t28979, t28990, t28992, t28995, t28999, t29001, t29003, t29004, t29008)
}
