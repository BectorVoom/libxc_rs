//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1213/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1213<F: Float>(t1821: F, t21115: F, t234: F, t5267: F, t410: F, t5845: F, t1871: F, t171: F, t1726: F, t18806: F, t230: F, t424: F, t5866: F, t718: F, t166: F, t1727: F, t5325: F, t5397: F) -> (F, F, F, F, F, F) {
    let t22379 = 0.12304822629859687989e6 * t234 * t5267 * t1821 * t21115;
    let t22382 = 16.0 * t410 * t5845;
    let t22383 = t1871 * t1871;
    let t22386 = 0.5143752e0 * t1726 * t171 * t22383;
    let t22387 = t18806 * t230;
    let t22390 = t424 * t718 * t5866;
    let t22395 = 0.41150016e1 * t5397 * t166 * t5325 * t1727;
    (t22379, t22382, t22386, t22387, t22390, t22395)
}
