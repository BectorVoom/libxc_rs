//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1011/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1011<F: Float>(t11670: F, t2124: F, t25183: F, t10698: F, t11702: F, t25192: F, t3295: F, t10792: F, t2201: F, t3613: F, t10760: F, t22790: F, t25577: F, t10885: F, t11744: F, t10888: F, t30792: F) -> (F, F, F, F, F, F, F) {
    let t39509 = t11670 * t2124 * t25183;
    let t39511 = t10698 * t11702;
    let t39512 = 0.12805040077930161442e0 * t39511;
    let t39514 = t3295 * t2124 * t25192;
    let t39517 = t2201 * t3613 * t10792;
    let t39520 = t22790 * t10760 * t25577;
    let t39522 = t11744 * t10885;
    let t39523 = 0.23115257973478049502e0 * t39522;
    let t39524 = t30792 * t10888;
    (t39509, t39512, t39514, t39517, t39520, t39523, t39524)
}
