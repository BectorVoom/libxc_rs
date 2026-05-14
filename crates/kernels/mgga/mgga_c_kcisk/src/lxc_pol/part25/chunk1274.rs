//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1274/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1274<F: Float>(t34668: F, t34671: F, t34674: F, t34677: F, t5218: F, t5339: F, t9988: F, t11694: F, t34303: F, t18175: F, t2799: F, t113203: F, t17784: F, t47033: F, t5219: F, t9967: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116061 = t34668 / 8.0;
    let t116062 = t34671 / 8.0;
    let t116063 = t34674 / 8.0;
    let t116064 = t34677 / 8.0;
    let t116067 = 2.0 * t5218 * t9988 * t5339;
    let t116069 = 4.0 * t11694 * t34303;
    let t116072 = 2.0 * t5218 * t2799 * t18175;
    let t116074 = 6.0 * t113203 * t17784;
    let t116077 = 24.0 * t47033 * t9967 * t5219;
    (t116061, t116062, t116063, t116064, t116067, t116069, t116072, t116074, t116077)
}
