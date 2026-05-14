//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 509/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk509<F: Float>(t4260: F, t4262: F, t3954: F, t556: F, t572: F, t1533: F, t4134: F, t4136: F, t571: F, t1494: F, t3722: F, t4108: F, t552: F, t577: F, t585: F, t3733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4263 = t4260 * t4262;
    let t4265 = t556 * t3954;
    let t4266 = t572 * t4265;
    let t4267 = t1533 * t4266;
    let t4269 = t4134 * t4136;
    let t4270 = t572 * t4269;
    let t4271 = t571 * t4270;
    let t4273 = t1494 * t3722;
    let t4274 = t572 * t4273;
    let t4275 = t571 * t4274;
    let t4277 = t4108 * t552;
    let t4278 = t4277 * t577;
    let t4279 = t4278 * t585;
    let t4281 = t3733 * t577;
    (t4263, t4265, t4266, t4267, t4269, t4270, t4271, t4273, t4274, t4275, t4277, t4278, t4279, t4281)
}
