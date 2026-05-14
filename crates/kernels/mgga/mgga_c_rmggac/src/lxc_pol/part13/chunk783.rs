//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 783/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk783<F: Float>(t1605: F, t1986: F, t7720: F, t36787: F, t8571: F, t35559: F, t35018: F, t36740: F, t9222: F, t118: F, t128: F, t1494: F, t209: F, t7474: F, t1970: F, t1971: F, t236: F, t5615: F) -> (F, F, F, F, F, F, F) {
    let t39490 = t1986 * t1605;
    let t39491 = t7720 * t39490;
    let t39493 = t8571 * t36787;
    let t39495 = t8571 * t35559;
    let t39497 = t8571 * t35018;
    let t39499 = t9222 * t36740;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39514 = t7474 * t39513;
    let t39518 = t1970 * t1971 * t236 * t5615;
    (t39491, t39493, t39495, t39497, t39499, t39514, t39518)
}
