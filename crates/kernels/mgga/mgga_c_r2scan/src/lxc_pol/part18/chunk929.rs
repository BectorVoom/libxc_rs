//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 929/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk929<F: Float>(t1337: F, t1347: F, t260: F, t277: F, t481: F, t1541: F, t57: F, t2146: F, t2182: F, t146: F, t6091: F, t774: F, t537: F, t252: F, t545: F, t6394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19309 = 1.0 / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = 1.0 / t19326;
    let t19790 = t260 * t277;
    let t19791 = t19790 * t481;
    let t19839 = t57 * t1541;
    let t19865 = t2182 * t2146;
    let t19872 = t146 * t6091 * t774;
    let t19875 = t1541 * t537;
    let t19877 = t146 * t19875 * t252;
    let t19883 = t545 * t6394;
    (t19309, t19327, t19790, t19791, t19839, t19865, t19872, t19877, t19883)
}
