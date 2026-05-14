//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1001/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1001<F: Float>(t17733: F, t1791: F, t10409: F, t6663: F, t11676: F, t2063: F, t4972: F, t5192: F, t5182: F, t2571: F, t4644: F, t6987: F, t5063: F, t5185: F, t10365: F, t6669: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17734 = t17733 * t1791;
    let t17739 = t10409 * t6663;
    let t17740 = 0.14739506172839506172e-2 * t17739;
    let t17742 = t11676 * t2063 * t4972;
    let t17743 = t5192 * t17742;
    let t17744 = t5182 * t17743;
    let t17746 = t2571 * t4644;
    let t17747 = t5192 * t17746;
    let t17748 = t5182 * t17747;
    let t17750 = t10409 * t6987;
    let t17751 = 0.22109259259259259258e-2 * t17750;
    let t17753 = t5185 * t2063 * t5063;
    let t17754 = t10365 * t17753;
    let t17755 = t5182 * t17754;
    let t17757 = t10409 * t6669;
    (t17734, t17739, t17740, t17742, t17744, t17746, t17748, t17750, t17751, t17753, t17755, t17757)
}
