//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 554/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk554<F: Float>(t2282: F, t6241: F, t4170: F, t196: F, t7828: F, t4271: F, t4272: F, t7706: F, t1471: F, t2059: F, t6298: F, t4282: F, t1472: F, t7710: F, t416: F, t8159: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8188 = 2.0 * t6241 * t2282;
    let t8189 = t2282 * t2282;
    let t8191 = 2.0 * t4170 * t8189;
    let t8192 = t7828 * t196;
    let t8212 = t4271 * t4272 * t7706;
    let t8216 = t1471 * t6298 * t2059;
    let t8220 = t1471 * t4282 * t7706;
    let t8224 = t1471 * t1472 * t7710;
    let t8227 = t416 * t8159;
    (t8188, t8189, t8191, t8192, t8212, t8216, t8220, t8224, t8227)
}
