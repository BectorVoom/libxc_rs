//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1272/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272<F: Float>(t15569: F, t18371: F, t19051: F, t4993: F, t11784: F, t1227: F, t21762: F, t248: F, t1174: F, t135: F, t22128: F, t22132: F) -> (F, F, F, F, F) {
    let t72542 = t15569 * t18371;
    let t72556 = t19051 * t4993;
    let t72560 = t1227 * t248 * t11784 * t21762;
    let t72597 = t1174 * t135 * t22128;
    let t72600 = t1174 * t135 * t22132;
    (t72542, t72556, t72560, t72597, t72600)
}
