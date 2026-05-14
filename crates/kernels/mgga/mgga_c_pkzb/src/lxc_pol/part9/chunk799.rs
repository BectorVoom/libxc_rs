//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 799/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk799<F: Float>(t5626: F, t5677: F, t5924: F, t5987: F, t158: F, t789: F, t2119: F, t799: F, t2118: F, t2145: F, t306: F, t5952: F, t5726: F, t5728: F, t2021: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5989 = t5626 + t5677 + t5924 + t5987;
    let t5990 = t5989 * t158;
    let t5999 = t789 * t789;
    let t6000 = 1.0 / t5999;
    let t6001 = t2119 * t799;
    let t6002 = t6000 * t6001;
    let t6005 = t2118 * t799;
    let t6006 = t6005 * t2145;
    let t6009 = t5952 * t306;
    let t6010 = t5726 * t5728;
    let t6012 = 1.0 / t2021 / t271;
    (t5989, t5990, t5999, t6000, t6001, t6002, t6006, t6009, t6010, t6012)
}
