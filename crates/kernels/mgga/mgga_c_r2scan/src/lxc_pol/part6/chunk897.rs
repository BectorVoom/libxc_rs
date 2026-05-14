//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 897/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk897<F: Float>(t551: F, t552: F, t6370: F, t108: F, t6359: F, t529: F, t6364: F, t1568: F, t1554: F, t1632: F, t574: F, t5074: F, t2155: F, t5174: F, t2145: F, t774: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6372 = t551 * t552 * t6370;
    let t6375 = t108 * t6359;
    let t6377 = t529 * t6375 * t6364;
    let t6381 = t529 * t1568 * t6370;
    let t6385 = t551 * t1632 * t1554;
    let t6386 = t574 * t6385;
    let t6389 = t551 * t552 * t5074;
    let t6392 = t2155 * t5174;
    let t6394 = t2145 * t774;
    (t6372, t6375, t6377, t6381, t6385, t6386, t6389, t6392, t6394)
}
