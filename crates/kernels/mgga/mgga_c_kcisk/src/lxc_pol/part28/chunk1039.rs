//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1039/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1039<F: Float>(t15897: F, t23913: F, t5182: F, t23912: F, t6758: F, t6674: F, t1692: F, t8616: F, t4581: F, t8874: F, t1869: F, t11237: F, t8673: F, t4817: F, t8677: F, t6961: F, t6974: F) -> (F, F, F, F, F, F, F, F) {
    let t23914 = t15897 * t23913;
    let t23915 = t5182 * t23914;
    let t23918 = t6758 * t23912;
    let t23919 = t15897 * t23918;
    let t23920 = t6674 * t23919;
    let t23922 = t8616 * t1692;
    let t23929 = t4581 * t8874;
    let t23930 = t1869 * t23929;
    let t23932 = t11237 * t8673;
    let t23933 = t1869 * t23932;
    let t23935 = t4817 * t8677;
    let t23936 = t1869 * t23935;
    let t23938 = t6974 * t6961;
    (t23915, t23918, t23920, t23922, t23930, t23933, t23936, t23938)
}
