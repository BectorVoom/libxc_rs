//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 966/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk966<F: Float>(t1254: F, t26221: F, t13526: F, t13686: F, t20292: F, t20302: F, t20531: F, t20532: F, t26138: F, t26141: F, t26144: F, t26147: F, t26150: F, t26153: F, t26156: F, t26159: F, t26162: F, t26165: F, t26168: F) -> (F, F) {
    let t26222 = t26221 * t1254;
    let t26241 = -t13686 - 0.41203703703703703703e-2 * t13526 - 0.82407407407407407408e-2 * t20292 + t20531 - t20532 + 0.12361111111111111111e-1 * t20302 + 0.20601851851851851852e-2 * t26138 - 0.10300925925925925926e-1 * t26141 + 0.37083333333333333333e-1 * t26144 - 0.24722222222222222222e-1 * t26147 - 0.61805555555555555557e-2 * t26150 - 0.55625000000000000001e-1 * t26153 + 0.74166666666666666668e-1 * t26156 + 0.30902777777777777778e-2 * t26159 - 0.61805555555555555555e-2 * t26162 + 0.18541666666666666667e-1 * t26165 - 0.92708333333333333333e-2 * t26168;
    (t26222, t26241)
}
