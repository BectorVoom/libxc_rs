//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1029/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1029<F: Float>(t5257: F, t8906: F, t6966: F, t8911: F, t17053: F, t3418: F, t8897: F, t1769: F, t8823: F, t8827: F, t8983: F, t6892: F, t8959: F, t9005: F, t16402: F, t3413: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24320 = t5257 * t8906;
    let t24322 = t6966 * t8911;
    let t24347 = t17053 * t3418;
    let t24370 = t5257 * t8897;
    let t24381 = t1769 * t8823;
    let t24387 = t1769 * t8827;
    let t24402 = t5257 * t8983;
    let t24421 = t6892 * t8959;
    let t24461 = t5257 * t9005;
    let t24487 = t16402 * t3413;
    (t24320, t24322, t24347, t24370, t24381, t24387, t24402, t24421, t24461, t24487)
}
