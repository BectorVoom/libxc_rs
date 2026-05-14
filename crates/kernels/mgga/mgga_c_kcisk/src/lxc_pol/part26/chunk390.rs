//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 390/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk390<F: Float>(t1340: F, t2236: F, t1411: F, t1220: F, t1335: F, t2110: F, t2174: F, t2179: F, t2215: F, t2234: F, t412: F, t504: F, t196: F) -> (F, F, F, F, F) {
    let t2237 = t1340 * t2236;
    let t2238 = t1411 * t2237;
    let t2240 = t2110 * t412 - 0.193e0 * t1220 * t2174 + t1335 + 0.16581944444444444444e-2 * t2179 + 0.24872916666666666666e-2 * t2215 - 0.24872916666666666666e-2 * t2234 + 0.16581944444444444444e-2 * t2238;
    let t2241 = t2240 * t504;
    let t2242 = t2110 * t196;
    (t2237, t2238, t2240, t2241, t2242)
}
