//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 989/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk989<F: Float>(t22430: F, t22510: F, t22641: F, t22950: F, t1801: F, t1873: F, t1869: F, t220: F, t2527: F, t5185: F, t5184: F, t15903: F, t15862: F, t6668: F, t5182: F, t6676: F) -> (F, F, F, F, F, F, F) {
    let t22952 = t22430 + t22510 + t22641 + t22950;
    let t22953 = t1801 * t22952;
    let t22954 = t1873 * t22953;
    let t22955 = t1869 * t22954;
    let t22957 = t220 * t2527;
    let t22958 = t5185 * t22957;
    let t22959 = t5184 * t22958;
    let t22960 = t15903 * t22959;
    let t22962 = t15862 * t6668;
    let t22963 = t5182 * t22962;
    let t22965 = t15862 * t6676;
    (t22952, t22953, t22955, t22958, t22960, t22963, t22965)
}
