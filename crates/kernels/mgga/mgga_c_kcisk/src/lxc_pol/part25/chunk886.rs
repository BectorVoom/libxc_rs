//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 886/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk886<F: Float>(t15921: F, t6675: F, t5192: F, t6674: F, t10459: F, t704: F, t1336: F, t140: F, t10463: F, t719: F, t2063: F, t3290: F) -> (F, F, F, F, F) {
    let t15922 = t6675 * t15921;
    let t15923 = t5192 * t15922;
    let t15924 = t6674 * t15923;
    let t15926 = t10459 * t704;
    let t15928 = t140 * t1336 * t15926;
    let t15929 = t719 * t10463;
    let t15930 = t2063 * t3290;
    (t15922, t15924, t15928, t15929, t15930)
}
