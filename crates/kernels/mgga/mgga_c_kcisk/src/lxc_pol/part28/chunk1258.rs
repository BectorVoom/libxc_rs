//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1258/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1258<F: Float>(t35551: F, t9904: F, t9907: F, t9910: F, t2819: F, t8472: F, t564: F, t11200: F, t1782: F, t4826: F, t5030: F, t3179: F, t3185: F, t1009: F, t15451: F, t10334: F, t195: F, t217: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35552 = t35551 / 8.0;
    let t35553 = t9904 * t9907;
    let t35554 = t35553 / 8.0;
    let t35555 = t9904 * t9910;
    let t35556 = t35555 / 8.0;
    let t35557 = t8472 * t2819;
    let t35558 = t564 * t35557;
    let t35559 = t35558 / 16.0;
    let t36247 = t1782 * t11200;
    let t36267 = t5030 * t4826;
    let t37229 = t3179 * t3185;
    let t37234 = t15451 * t1009;
    let t43141 = t195 / t10334 / t217;
    (t35552, t35554, t35556, t35559, t36247, t36267, t37229, t37234, t43141)
}
