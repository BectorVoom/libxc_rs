//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 865/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk865<F: Float>(t14223: F, t2237: F, t3739: F, t5982: F, t2233: F, t5976: F, t12951: F, t470: F, t12825: F, t453: F, t1336: F, t140: F, t12829: F, t1337: F, t2211: F) -> (F, F, F, F, F, F, F, F) {
    let t19020 = t14223 * t2237;
    let t19022 = t3739 * t5982;
    let t19028 = t14223 * t2233;
    let t19030 = t3739 * t5976;
    let t19032 = t470 * t12951;
    let t19044 = t12825 * t453;
    let t19046 = t140 * t1336 * t19044;
    let t19047 = t470 * t12829;
    let t19053 = t1337 * t2211;
    let t19055 = t140 * t1336 * t19053;
    (t19020, t19022, t19028, t19030, t19032, t19046, t19047, t19055)
}
