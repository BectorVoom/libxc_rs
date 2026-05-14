//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1391/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1391<F: Float>(t1333: F, t35202: F, t116145: F, t1869: F, t34085: F, t34089: F, t34093: F, t1799: F, t6662: F, t22953: F, t415: F, t9687: F, t1864: F, t8878: F, t8882: F, t23053: F, t717: F) -> (F, F, F, F, F, F, F, F) {
    let t122021 = t1333 * t35202;
    let t122024 = t1869 * t116145 * t34085;
    let t122029 = t1869 * t34093 * t34089;
    let t122036 = t1799 * t34093 * t6662;
    let t122041 = t415 * t9687 * t22953;
    let t122044 = t415 * t1864 * t8878;
    let t122047 = t415 * t1864 * t8882;
    let t122050 = t415 * t717 * t23053;
    (t122021, t122024, t122029, t122036, t122041, t122044, t122047, t122050)
}
