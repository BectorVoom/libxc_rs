//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 899/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk899<F: Float>(t3495: F, t3512: F, t1411: F, t3494: F, t3502: F, t1340: F, t3503: F, t10519: F, t10520: F, t12924: F, t8: F, t1450: F) -> (F, F, F, F, F) {
    let t13412 = t3512 * t3495;
    let t13413 = t1411 * t13412;
    let t13415 = t3494 * t3502;
    let t13416 = t1340 * t13415;
    let t13417 = t1411 * t13416;
    let t13419 = t3512 * t3503;
    let t13420 = t1411 * t13419;
    let t13423 = t12924 * t8 - t10519 + t10520;
    let t13424 = t1450 * t13423;
    (t13413, t13417, t13420, t13423, t13424)
}
