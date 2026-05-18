//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 640/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk640<F: Float>(t270: F, t535: F, t2039: F, t638: F, t31: F, t2046: F, t2050: F, t2305: F, t7255: F, t236: F, t5605: F, t1971: F) -> (F, F, F, F, F, F) {
    let t8486 = t535 * t270;
    let t8488 = t638 * t2039 * t8486;
    let t8490 = t535 * t31;
    let t8492 = t2046 * t2050 * t8490;
    let t8494 = t7255 * t2305;
    let t8496 = t236 * t5605;
    let t8497 = t1971 * t8496;
    (t8486, t8488, t8490, t8492, t8494, t8497)
}
