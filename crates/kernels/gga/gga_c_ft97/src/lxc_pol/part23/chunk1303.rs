//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1303/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1303<F: Float>(t245: F, t121708: F, t121736: F, t121769: F, t121807: F, t121840: F, t121875: F, t125105: F, t125143: F, t125178: F, t125205: F, t125241: F, t125271: F, t125296: F, t125319: F, t125362: F, t125391: F, t1459: F, t15625: F, t1577: F, t18: F, t21: F, t28474: F, t31323: F, t363: F, t4431: F, t5: F, t6200: F, t6953: F, t920: F) -> (F,) {
    let t246 = 10000000.0 <= t245;
    let t125415 = piecewise3(t246, 0.0, t5 * (t121708 + t121736 + t121769 + t121807 + t121840 + t121875 + t125105 + t125143 + t125178 + t125205 + t125241 + t125271 + t125296 + t125319 + t125362 + t125391) * t21 / 4.0 + t5 * t31323 * t363 / 4.0 + t5 * t28474 * t920 / 2.0 + t5 * t6953 * t18 * t1577 + t5 * t6200 * t4431 / 4.0 + t5 * t1459 * t15625 / 4.0);
    (t125415,)
}
