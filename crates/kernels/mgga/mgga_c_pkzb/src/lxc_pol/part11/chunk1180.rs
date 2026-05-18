//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1180/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1180<F: Float>(t10586: F, t5257: F, t10582: F, t10627: F, t1721: F, t600: F, t10578: F, t6892: F, t10574: F, t6966: F, t5391: F, t10621: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28999 = t5257 * t10586;
    let t29001 = t5257 * t10582;
    let t29003 = t10627 * t1721;
    let t29004 = t29003 * t600;
    let t29008 = t6892 * t10578;
    let t29010 = t6966 * t10574;
    let t29012 = t10627 * t5391;
    let t29013 = t29012 * t600;
    let t29017 = t10621 * t1721;
    (t28999, t29001, t29003, t29004, t29008, t29010, t29012, t29013, t29017)
}
