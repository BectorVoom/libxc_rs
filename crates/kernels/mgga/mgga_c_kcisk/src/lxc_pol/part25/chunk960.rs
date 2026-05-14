//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 960/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk960<F: Float>(t10409: F, t6681: F, t16676: F, t5188: F, t4811: F, t6962: F, t6967: F, t15851: F, t1900: F, t1869: F, t5194: F, t6758: F, t15897: F, t6674: F, t10364: F, t1801: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17016 = t10409 * t6681;
    let t17018 = t16676 * t5188;
    let t17020 = t4811 * t6962;
    let t17021 = 0.88437037037037037034e-2 * t17020;
    let t17022 = t4811 * t6967;
    let t17024 = t15851 * t1900;
    let t17025 = t1869 * t17024;
    let t17027 = t6758 * t5194;
    let t17028 = t15897 * t17027;
    let t17029 = t6674 * t17028;
    let t17031 = t10364 * t1801;
    (t17016, t17018, t17020, t17021, t17022, t17025, t17027, t17029, t17031)
}
