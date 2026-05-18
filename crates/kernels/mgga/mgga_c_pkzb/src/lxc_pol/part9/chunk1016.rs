//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1016/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1016<F: Float>(t2382: F, t6416: F, t8254: F, t824: F, t919: F, t2371: F, t300: F, t3236: F, t2383: F, t2185: F, t3175: F, t2888: F) -> (F, F, F, F, F, F, F, F) {
    let t8255 = t6416 * t2382;
    let t8256 = t8254 * t8255;
    let t8259 = t919 * t824;
    let t8260 = t2371 * t8259;
    let t8261 = t8254 * t8260;
    let t8264 = t300 * t3236;
    let t8265 = t8264 * t2383;
    let t8269 = t3175 * t2185;
    let t8270 = t2888 * t8269;
    (t8255, t8256, t8260, t8261, t8264, t8265, t8269, t8270)
}
