//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 709/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk709<F: Float>(t2656: F, t5531: F, t14223: F, t2237: F, t2233: F, t1337: F, t2211: F, t1336: F, t140: F, t2076: F, t2869: F) -> (F, F, F, F, F) {
    let t18925 = t2656 * t5531;
    let t19020 = t14223 * t2237;
    let t19028 = t14223 * t2233;
    let t19053 = t1337 * t2211;
    let t19055 = t140 * t1336 * t19053;
    let t19100 = t2869 * t2076;
    (t18925, t19020, t19028, t19055, t19100)
}
