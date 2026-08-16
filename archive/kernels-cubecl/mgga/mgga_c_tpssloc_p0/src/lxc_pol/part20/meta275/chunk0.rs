//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1444/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1444<F: Float>(t10481: F, t10482: F, t1021: F, t248: F, t2776: F, t3051: F, t1041: F, t10316: F, t1044: F, t3103: F, t3109: F, t10309: F, t3062: F) -> (F, F, F, F, F, F, F) {
    let t10483 = t10481 * t10482;
    let t10485 = t248 * t1021 * t10483;
    let t10489 = t248 * t3051 * t2776;
    let t10490 = t1041 * t10489;
    let t10493 = t248 * t1044 * t10316;
    let t10496 = t3109 * t3103;
    let t10501 = t248 * t3062 * t10309;
    (t10483, t10485, t10489, t10490, t10493, t10496, t10501)
}
