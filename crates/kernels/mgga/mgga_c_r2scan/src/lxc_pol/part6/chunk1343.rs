//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1343/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1343<F: Float>(t20594: F, t2691: F, t6087: F, t13866: F, t2195: F, t6064: F, t2183: F, t2687: F, t1632: F, t551: F, t6218: F, t8170: F, t5054: F, t910: F, t6528: F, t8048: F) -> (F, F, F, F, F, F) {
    let t25357 = t20594 * t2691 * t6087;
    let t25359 = t2195 * t13866;
    let t25361 = t25359 * t2691 * t6064;
    let t25363 = t2183 * t13866;
    let t25365 = t25363 * t2687 * t6087;
    let t25369 = t6218 * t551 * t1632 * t8170;
    let t25372 = t910 * t5054;
    let t25379 = t6528 * t551 * t1632 * t8048;
    (t25357, t25361, t25365, t25369, t25372, t25379)
}
