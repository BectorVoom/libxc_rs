//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1172/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1172<F: Float>(t18914: F, t230: F, t21519: F, t61: F, t21432: F, t4885: F, t661: F, t1726: F, t5364: F, t607: F, t1819: F, t1821: F, t21430: F, t234: F, t21115: F, t5267: F) -> (F, F, F, F, F, F, F) {
    let t22350 = 840.0 * t18914 * t230;
    let t22352 = 0.57791679765211885293e1 * t61 * t21519;
    let t22354 = 0.3903689268108626343e0 * t61 * t21432;
    let t22360 = t4885 * t661;
    let t22365 = t1726 * t607 * t5364;
    let t22375 = 0.30762056574649219974e4 * t234 * t1819 * t1821 * t21430;
    let t22379 = 0.12304822629859687989e6 * t234 * t5267 * t1821 * t21115;
    (t22350, t22352, t22354, t22360, t22365, t22375, t22379)
}
