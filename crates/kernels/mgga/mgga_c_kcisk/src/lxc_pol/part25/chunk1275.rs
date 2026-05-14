//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1275/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1275<F: Float>(t11701: F, t5339: F, t9967: F, t1957: F, t34306: F, t34374: F, t5213: F, t112011: F, t7296: F, t17790: F, t33071: F, t11694: F, t34310: F, t5218: F, t7444: F, t9718: F) -> (F, F, F, F, F, F, F) {
    let t116080 = 6.0 * t11701 * t9967 * t5339;
    let t116083 = 12.0 * t11701 * t34306 * t1957;
    let t116085 = 2.0 * t5213 * t34374;
    let t116087 = 4.0 * t112011 * t7296;
    let t116089 = 2.0 * t33071 * t17790;
    let t116091 = 4.0 * t11694 * t34310;
    let t116094 = 4.0 * t5218 * t9718 * t7444;
    (t116080, t116083, t116085, t116087, t116089, t116091, t116094)
}
