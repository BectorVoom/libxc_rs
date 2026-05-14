//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1173/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1173<F: Float>(t410: F, t5845: F, t1871: F, t171: F, t1726: F, t18806: F, t230: F, t424: F, t5866: F, t718: F, t166: F, t1727: F, t5325: F, t5397: F, t234: F, t5429: F, t704: F, t740: F) -> (F, F, F, F, F, F) {
    let t22382 = 16.0 * t410 * t5845;
    let t22383 = t1871 * t1871;
    let t22386 = 0.5143752e0 * t1726 * t171 * t22383;
    let t22387 = t18806 * t230;
    let t22390 = t424 * t718 * t5866;
    let t22395 = 0.41150016e1 * t5397 * t166 * t5325 * t1727;
    let t22399 = 0.46785788981077169656e1 * t234 * t704 * t5429 * t740;
    (t22382, t22386, t22387, t22390, t22395, t22399)
}
