//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 913/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk913<F: Float>(t10409: F, t6681: F, t4811: F, t6962: F, t6967: F, t10364: F, t1801: F, t5183: F, t6975: F, t2507: F, t5060: F, t6978: F, t1871: F, t6943: F, t7071: F, t6970: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17016 = t10409 * t6681;
    let t17020 = t4811 * t6962;
    let t17021 = 0.88437037037037037034e-2 * t17020;
    let t17022 = t4811 * t6967;
    let t17031 = t10364 * t1801;
    let t17044 = t5183 * t1801;
    let t17054 = t4811 * t6975;
    let t17055 = 0.33163888888888888888e-2 * t17054;
    let t17056 = t2507 * t5060;
    let t17057 = t17056 * sigma2;
    let t17061 = t4811 * t6978;
    let t17064 = t6943 * t1871;
    let t17065 = t17064 * sigma2;
    let t17069 = t4811 * t7071;
    let t17070 = 0.33163888888888888888e-2 * t17069;
    let t17076 = t4811 * t6970;
    (t17016, t17020, t17021, t17022, t17031, t17044, t17054, t17055, t17056, t17057, t17061, t17064, t17065, t17069, t17070, t17076)
}
