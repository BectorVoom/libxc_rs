//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 880/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk880<F: Float>(t15852: F, t1799: F, t5048: F, t6719: F, t2454: F, t5183: F, t5187: F, t5182: F, t5191: F, t5195: F, t5074: F, t6699: F, t79: F, t918: F, t140: F, t1797: F) -> (F, F, F, F, F, F) {
    let t15853 = t1799 * t15852;
    let t15855 = t6719 * t5048;
    let t15856 = t1799 * t15855;
    let t15858 = t5183 * t2454;
    let t15859 = t15858 * t5187;
    let t15860 = t5182 * t15859;
    let t15862 = t5191 * t2454;
    let t15863 = t15862 * t5195;
    let t15864 = t5182 * t15863;
    let t15866 = t5074 * t6699;
    let t15868 = t918 * t79;
    let t15870 = t140 * t15868 * t1797;
    (t15853, t15856, t15860, t15864, t15866, t15870)
}
