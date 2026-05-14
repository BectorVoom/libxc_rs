//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 881/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk881<F: Float>(t14570: F, t486: F, t13777: F, t4143: F, t487: F, t13288: F, t499: F, t498: F, t1504: F, t4214: F, t469: F, t4205: F, t1513: F, t4301: F, t1501: F, t4182: F) -> (F, F, F, F, F, F) {
    let t14571 = t486 * t14570;
    let t14573 = t4143 * t13777;
    let t14574 = t487 * t14573;
    let t14575 = t486 * t14574;
    let t14577 = t499 * t13288;
    let t14578 = t498 * t14577;
    let t14579 = t1504 * t14578;
    let t14581 = t4214 * t469;
    let t14582 = t14581 * t4205;
    let t14584 = t4301 * t1513;
    let t14586 = t1501 * t4182;
    (t14571, t14575, t14579, t14582, t14584, t14586)
}
