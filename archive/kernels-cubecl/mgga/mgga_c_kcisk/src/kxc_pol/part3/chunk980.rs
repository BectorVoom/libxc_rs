//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 980/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk980<F: Float>(t4265: F, t4288: F, t4274: F, t1471: F, t3283: F, t4277: F, t12924: F, t1472: F, t12868: F, t6287: F, t12983: F, t6279: F) -> (F, F, F, F, F, F) {
    let t14444 = t4265 * t4288;
    let t14446 = t4265 * t4274;
    let t14449 = t1471 * t4277 * t3283;
    let t14453 = t1471 * t1472 * t12924;
    let t14458 = t6287 * t12868;
    let t14461 = t6279 * t12983;
    (t14444, t14446, t14449, t14453, t14458, t14461)
}
