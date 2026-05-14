//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1200/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1200<F: Float>(t240: F, t34287: F, t34289: F, t34290: F, t34292: F, t34293: F, t34294: F, t34296: F, t34297: F, t34299: F, t34302: F, t34305: F, t34308: F, t34309: F, t34312: F, t34375: F, t34385: F, t34653: F) -> (F,) {
    let t34656 = t34287 - t34289 - t34290 + t34292 - t34293 - t34294 + t34296 - t34297 + t34299 - t34302 + t34305 + t34308 - t34309 + t34312 - t34375 + t240 * (t34385 + t34653);
    (t34656,)
}
