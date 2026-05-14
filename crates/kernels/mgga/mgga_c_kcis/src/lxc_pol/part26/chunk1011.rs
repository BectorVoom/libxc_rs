//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1011/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1011<F: Float>(t1489: F, t2038: F, t28503: F, t1464: F, t1014: F, t8176: F, t5649: F, t7923: F, t1394: F, t5655: F, t5663: F, t4153: F, t27387: F, t5644: F, t2237: F, t28426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28504 = t2038 * t1489;
    let t28505 = t28503 * t28504;
    let t28506 = t1464 * t28505;
    let t28508 = t1014 * t8176;
    let t28510 = t7923 * t5649;
    let t28511 = t1394 * t28510;
    let t28513 = t7923 * t5655;
    let t28514 = t1394 * t28513;
    let t28516 = t7923 * t5663;
    let t28517 = t4153 * t28516;
    let t28519 = t27387 * t5644;
    let t28520 = t1394 * t28519;
    let t28522 = t2237 * t28426;
    (t28504, t28505, t28506, t28508, t28510, t28511, t28513, t28514, t28516, t28517, t28519, t28520, t28522)
}
