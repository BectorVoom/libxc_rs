//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 792/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk792<F: Float>(t28578: F, t28698: F, t673: F, t716: F, t720: F, t415: F, t2533: F, t8666: F, t6719: F, t8874: F, t1869: F, t23033: F, t2527: F, t1873: F, t23038: F, t2441: F) -> (F, F, F, F, F, F) {
    let t28699 = t28578 + t28698;
    let t28700 = t673 * t28699;
    let t28701 = t28700 * t716;
    let t28702 = t28701 * t720;
    let t28703 = t415 * t28702;
    let t28705 = t8666 * t2533;
    let t28706 = t415 * t28705;
    let t28710 = t6719 * t8874;
    let t28711 = t1869 * t28710;
    let t28713 = t23033 * t2527;
    let t28714 = t1873 * t28713;
    let t28715 = t1869 * t28714;
    let t28717 = t23038 * t2441;
    (t28699, t28703, t28706, t28711, t28715, t28717)
}
