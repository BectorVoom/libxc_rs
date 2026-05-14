//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 727/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk727<F: Float>(t2405: F, t934: F, t638: F, t7292: F, t8475: F, t1591: F, t2039: F, t270: F, t2338: F, t7323: F, t7324: F, t1327: F, t574: F, t640: F, t34750: F, t34755: F, t577: F) -> (F, F, F, F, F, F) {
    let t39320 = t934 * t2405;
    let t39333 = t638 * t7292 * t8475;
    let t39338 = t638 * t2039 * t1591 * t270;
    let t39339 = 0.30487649791575028314e-3 * t39338;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39370 = t34755 * t577 * t34750;
    (t39320, t39333, t39339, t39341, t39345, t39370)
}
