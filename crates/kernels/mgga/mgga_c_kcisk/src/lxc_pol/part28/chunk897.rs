//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 897/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk897<F: Float>(t2454: F, t5183: F, t5191: F, t5074: F, t6699: F, t79: F, t918: F, t140: F, t1797: F, t6716: F, t1336: F, t705: F, t2527: F, t642: F, t1899: F, t5180: F, t5598: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15858 = t5183 * t2454;
    let t15862 = t5191 * t2454;
    let t15866 = t5074 * t6699;
    let t15868 = t918 * t79;
    let t15870 = t140 * t15868 * t1797;
    let t15871 = t15870 * t6716;
    let t15891 = t140 * t1336 * t705;
    let t15892 = t642 * t2527;
    let t15897 = t5191 * t1899;
    let t15903 = t140 * t5598 * t5180;
    (t15858, t15862, t15866, t15870, t15871, t15891, t15892, t15897, t15903)
}
