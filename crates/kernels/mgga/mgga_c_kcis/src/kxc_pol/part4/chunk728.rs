//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 728/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk728<F: Float>(t4129: F, t4261: F, t4260: F, t3954: F, t556: F, t572: F, t1533: F, t4134: F, t4136: F, t571: F, t1494: F, t3722: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4262 = t4261 * t4129;
    let t4263 = t4260 * t4262;
    let t4265 = t556 * t3954;
    let t4266 = t572 * t4265;
    let t4267 = t1533 * t4266;
    let t4269 = t4134 * t4136;
    let t4270 = t572 * t4269;
    let t4271 = t571 * t4270;
    let t4273 = t1494 * t3722;
    (t4262, t4263, t4265, t4266, t4267, t4269, t4270, t4271, t4273)
}
