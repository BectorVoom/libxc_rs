//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 967/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk967<F: Float>(t11213: F, t17132: F, t1800: F, t1869: F, t5063: F, t6697: F, t5062: F, t2571: F, t4640: F, t5192: F, t6674: F, t10414: F, t6681: F, t1636: F, t7405: F, t5184: F) -> (F, F, F, F, F, F, F, F) {
    let t17133 = t11213 * t17132;
    let t17134 = t1800 * t17133;
    let t17135 = t1869 * t17134;
    let t17137 = t6697 * t5063;
    let t17138 = t5062 * t17137;
    let t17139 = t1869 * t17138;
    let t17141 = t2571 * t4640;
    let t17142 = t5192 * t17141;
    let t17143 = t6674 * t17142;
    let t17150 = t10414 * t6681;
    let t17152 = t7405 * t1636;
    let t17153 = t5184 * t17152;
    (t17135, t17137, t17139, t17141, t17143, t17150, t17152, t17153)
}
