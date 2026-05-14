//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 817/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk817<F: Float>(t6121: F, t8820: F, t360: F, t1569: F, t2530: F, t2572: F, t2124: F, t9317: F, t2590: F, t259: F, t8196: F, t571: F, t1567: F, t3052: F, t2591: F, t8778: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9501 = t8820 * t6121;
    let t9502 = t360 * t9501;
    let t9507 = t1569 * t2530;
    let t9508 = t2572 * t9507;
    let t9509 = t360 * t9508;
    let t9513 = t2124 * t9317 * t6121;
    let t9517 = t2124 * t2590 * t9507;
    let t9520 = t8196 * t259;
    let t9521 = t571 * t9520;
    let t9524 = t1567 * t3052;
    let t9526 = t2124 * t9524 * t2591;
    let t9529 = t8778 * t2591;
    (t9501, t9502, t9508, t9509, t9513, t9517, t9521, t9526, t9529)
}
