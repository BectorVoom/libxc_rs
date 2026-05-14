//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 660/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk660<F: Float>(t1684: F, t5402: F, t1680: F, t1678: F, t607: F, t159: F, t1686: F, t1745: F, t732: F, t1731: F, t5311: F, t5314: F, t636: F, t12: F, t3: F, t40: F) -> (F, F, F, F, F, F) {
    let t5403 = t1684 * t5402;
    let t5405 = 0.16936279733333333332e-2 * t1680 * t5403;
    let t5407 = t607 * t1678;
    let t5408 = t159 * t5407;
    let t5409 = t5408 * t1686;
    let t5413 = t732 * t1745;
    let t5416 = t1731 * t5311;
    let t5418 = t636 * t5314;
    let t5420 = f64::powf(t12, -0.25e1);
    let t5421 = t5420 * t3;
    let t5422 = t5421 * t40;
    (t5405, t5409, t5413, t5416, t5418, t5422)
}
