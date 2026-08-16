//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 779/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk779<F: Float>(t1114: F, t1501: F, t3068: F, t3090: F, t242: F, t1125: F, t2840: F, t3096: F, t3426: F, t3931: F, t1127: F, t2845: F) -> (F, F, F, F, F, F, F) {
    let t4270 = t1501 * t1114;
    let t4271 = t3068 * t4270;
    let t4274 = t3090 * t1501;
    let t4275 = t242 * t4274;
    let t4276 = t1125 * t4275;
    let t4278 = t3096 * t2840;
    let t4279 = t4278 * t3426;
    let t4280 = t3931 * t4279;
    let t4283 = t1127 * t2845;
    (t4270, t4271, t4276, t4278, t4279, t4280, t4283)
}
