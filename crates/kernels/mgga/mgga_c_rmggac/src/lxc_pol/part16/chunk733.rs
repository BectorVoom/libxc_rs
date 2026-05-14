//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 733/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk733<F: Float>(t1591: F, t2039: F, t270: F, t638: F, t2338: F, t7323: F, t7324: F, t1327: F, t574: F, t640: F, t34960: F, t34750: F, t34755: F, t577: F, t2339: F, t7184: F) -> (F, F, F, F, F, F) {
    let t39338 = t638 * t2039 * t1591 * t270;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39364 = 0.2927036860455597649e0 * t34960;
    let t39370 = t34755 * t577 * t34750;
    let t39388 = t638 * t7184 * t2339;
    (t39338, t39341, t39345, t39364, t39370, t39388)
}
