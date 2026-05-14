//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 909/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk909<F: Float>(t17648: F, t442: F, t441: F, t15274: F, t15311: F, t11940: F, t12119: F, t12122: F, t15322: F, t15324: F, t15330: F, t15333: F, t15336: F, t15598: F, t15620: F, t15623: F, t451: F, t5333: F, t8531: F, t8548: F, t8966: F) -> (F, F, F, F) {
    let t17885 = t442 * t17648;
    let t17886 = t441 * t17885;
    let t17893 = t15311 * t15274;
    let t17896 = 0.60369177012421929545e-2 * t15322 - 0.3863627328795003491e-1 * t15324 - 0.75734008510040627576e0 * t15330 + 0.71000632978163088351e-1 * t15333 + 0.36221506207453157727e-2 * t15336 - 0.72443012414906315455e-2 * t15598 + t12119 / 54.0 + 11.0 / 108.0 * t15620 - 0.67291509309846310801e0 * t17886 * t451 + 0.18314556960919660338e2 * t15623 - t12122 / 432.0 + t8531 + t8548 + 0.73258227843678641352e2 * t11940 * t5333 - 0.91572784804598301689e1 * t8966 * t17893;
    (t17885, t17886, t17893, t17896)
}
