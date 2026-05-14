//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 945/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk945<F: Float>(t4811: F, t6709: F, t4817: F, t6703: F, t1869: F, t2527: F, t4972: F, t5203: F, t1873: F, t5063: F, t1801: F, t11227: F, t1849: F, t2487: F, t3290: F, t4609: F) -> (F, F, F, F, F, F, F) {
    let t16702 = t4811 * t6709;
    let t16704 = t4817 * t6703;
    let t16705 = t1869 * t16704;
    let t16711 = t2527 * t4972;
    let t16712 = t5203 * t16711;
    let t16713 = t1873 * t16712;
    let t16714 = t1869 * t16713;
    let t16716 = t2527 * t5063;
    let t16717 = t1801 * t16716;
    let t16718 = t11227 * t16717;
    let t16719 = t1869 * t16718;
    let t16724 = t2487 * t1849 * t3290;
    let t16725 = t4609 * t16724;
    (t16702, t16705, t16711, t16714, t16716, t16719, t16725)
}
