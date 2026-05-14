//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 988/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk988<F: Float>(t26933: F, t7749: F, t1165: F, t1176: F, t283: F, t7755: F, t3190: F, t3338: F, t7754: F, t389: F, t9568: F, t3219: F, t5077: F, t26918: F, t26920: F, t26922: F, t26925: F, t26927: F, t26931: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26934 = t26933 * t7749;
    let t26936 = t1165 * t1176;
    let t26938 = t1165 * t283;
    let t26939 = t26938 * t7755;
    let t26941 = t3338 * t3190;
    let t26942 = t7754 * t26941;
    let t26944 = t9568 * t389;
    let t26946 = t5077 * t3219;
    let t26947 = t7754 * t26946;
    let t26949 = -t26918 / 16.0 + t26920 / 16.0 + 11.0 / 18.0 * t26922 - 2.0 / 9.0 * t26925 - t26927 / 12.0 + t26931 / 48.0 - t26934 / 8.0 - t26936 / 3.0 + t26939 / 12.0 + t26942 / 24.0 - t26944 / 128.0 - t26947 / 72.0;
    (t26934, t26936, t26938, t26939, t26941, t26942, t26944, t26946, t26947, t26949)
}
