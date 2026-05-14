//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1040/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1040<F: Float>(t1869: F, t23938: F, t6719: F, t6966: F, t1693: F, t17055: F, t17061: F, t1792: F, t22919: F, t23292: F, t23920: F, t23922: F, t23930: F, t23933: F, t23936: F, t671: F) -> (F, F, F) {
    let t23939 = t1869 * t23938;
    let t23941 = t6719 * t6966;
    let t23942 = t1869 * t23941;
    let t23944 = 0.3684876543209876543e-2 * t23920 - 0.193e0 * t23922 * t1792 + 0.386e0 * t1693 * t23292 + t22919 * t671 - t17055 + 0.22109259259259259259e-2 * t17061 - 0.55273148148148148147e-3 * t23930 + 0.49745833333333333332e-2 * t23933 + 0.13265555555555555555e-1 * t23936 + 0.13265555555555555555e-1 * t23939 - 0.88437037037037037033e-2 * t23942;
    (t23939, t23942, t23944)
}
