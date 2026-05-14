//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1023/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1023<F: Float>(t1254: F, t20522: F, t20295: F, t20298: F, t13526: F, t13530: F, t13533: F, t13536: F, t13686: F, t20292: F, t20302: F, t20305: F, t20308: F, t20312: F, t20315: F, t20318: F, t20321: F, t20324: F, t20327: F) -> (F, F) {
    let t20523 = t20522 * t1254;
    let t20531 = 0.41203703703703703704e-2 * t20295;
    let t20532 = 0.12361111111111111111e-1 * t20298;
    let t20542 = -t13686 - 0.82407407407407407407e-2 * t13526 + 0.20601851851851851852e-2 * t13530 - 0.61805555555555555556e-2 * t13533 + 0.30902777777777777778e-2 * t13536 - 0.41203703703703703704e-2 * t20292 + t20531 - t20532 + 0.67986111111111111113e-1 * t20302 - 0.10300925925925925926e-1 * t20305 + 0.37083333333333333333e-1 * t20308 - 0.24722222222222222222e-1 * t20312 - 0.61805555555555555555e-2 * t20315 - 0.55625000000000000001e-1 * t20318 + 0.74166666666666666668e-1 * t20321 + 0.18541666666666666667e-1 * t20324 - 0.18541666666666666667e-1 * t20327;
    (t20523, t20542)
}
