//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 978/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk978(t17648: f64, t442: f64, t441: f64, t15274: f64, t15311: f64, t11940: f64, t12119: f64, t12122: f64, t15322: f64, t15324: f64, t15330: f64, t15333: f64, t15336: f64, t15598: f64, t15620: f64, t15623: f64, t451: f64, t5333: f64, t8531: f64, t8548: f64, t8966: f64) -> (f64, f64, f64, f64) {
    let t17885 = t442 * t17648;
    let t17886 = t441 * t17885;
    let t17893 = t15311 * t15274;
    let t17896 = 0.60369177012421929545e-2_f64 * t15322 - 0.3863627328795003491e-1_f64 * t15324 - 0.75734008510040627576e0_f64 * t15330 + 0.71000632978163088351e-1_f64 * t15333 + 0.36221506207453157727e-2_f64 * t15336 - 0.72443012414906315455e-2_f64 * t15598 + t12119 / 54.0_f64 + 11.0_f64 / 108.0_f64 * t15620 - 0.67291509309846310801e0_f64 * t17886 * t451 + 0.18314556960919660338e2_f64 * t15623 - t12122 / 432.0_f64 + t8531 + t8548 + 0.73258227843678641352e2_f64 * t11940 * t5333 - 0.91572784804598301689e1_f64 * t8966 * t17893;
    (t17885, t17886, t17893, t17896)
}
