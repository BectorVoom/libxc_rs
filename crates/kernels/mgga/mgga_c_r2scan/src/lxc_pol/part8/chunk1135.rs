//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1135/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1135<F: Float>(t5686: F, t632: F, t645: F, t170: F, t5861: F, t5363: F, t591: F, t189: F, t5790: F, t621: F, t1679: F, t1684: F, t1685: F, t1726: F, t1727: F, t1672: F, t5228: F) -> (F, F, F, F, F, F, F) {
    let t21379 = 8.0 * t632 * t645 * t5686;
    let t21380 = t170 * t5861;
    let t21383 = 0.6858336e0 * t5363 * t21380 * t591;
    let t21384 = t189 * t5686;
    let t21387 = 24.0 * t5790 * t21384 * t621;
    let t21392 = 0.1524265176e-1 * t1726 * t1679 * t1684 * t1685 * t1727;
    let t21394 = t1726 * t1672 * t5228;
    (t21379, t21380, t21383, t21384, t21387, t21392, t21394)
}
