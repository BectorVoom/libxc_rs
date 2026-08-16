//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1271/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1271<F: Float>(t1464: F, t27423: F, t98470: F, t17254: F, t2243: F, t303: F, t2237: F, t54162: F, t8158: F, t1394: F, t15838: F, t27387: F) -> (F, F, F, F) {
    let t98754 = t1464 * t98470 * t27423;
    let t98767 = t303 * t17254 * t2243;
    let t98777 = t2237 * t54162 * t8158;
    let t98781 = t1394 * t27387 * t15838;
    (t98754, t98767, t98777, t98781)
}
