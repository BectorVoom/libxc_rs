//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 973/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk973<F: Float>(t1044: F, t1717: F, t588: F, t1123: F, t2003: F, t300: F, t5955: F, t759: F, t178: F, t8358: F, t2364: F, t2394: F, t2886: F, t980: F, t6517: F, t919: F) -> (F, F, F, F, F, F, F, F) {
    let t9056 = t1717 * t1044;
    let t9067 = t588 * t1044;
    let t9257 = t2003 * t1123;
    let t9258 = t300 * t9257;
    let t9319 = t5955 * t759;
    let t10043 = t8358 * t178;
    let t10044 = t2364 * t10043;
    let t10047 = t2394 * t10043;
    let t10063 = t980 * t2886;
    let t10121 = t6517 * t919;
    (t9056, t9067, t9258, t9319, t10044, t10047, t10063, t10121)
}
