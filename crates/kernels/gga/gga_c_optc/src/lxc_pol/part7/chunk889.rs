//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 889/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk889<F: Float>(t1162: F, t9084: F, t2367: F, t3088: F, t1781: F, t321: F, t429: F, t457: F, t8936: F, t914: F, t1122: F, t430: F, t3126: F, t4356: F, t4463: F, t8193: F) -> (F, F, F, F, F, F, F) {
    let t9085 = t1162 * t9084;
    let t9087 = t2367 * t3088;
    let t9088 = t1162 * t9087;
    let t9091 = t321 * t1781 * t429;
    let t9093 = 0.32196894406625029092e-1 * t457 * t9091;
    let t9094 = t914 * t8936;
    let t9097 = t430 * t1122;
    let t9098 = t4356 * t3126;
    let t9099 = t9097 * t9098;
    let t9102 = t4463 * t8193;
    (t9085, t9088, t9093, t9094, t9097, t9099, t9102)
}
