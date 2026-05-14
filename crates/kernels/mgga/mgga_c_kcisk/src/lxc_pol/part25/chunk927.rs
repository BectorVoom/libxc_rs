//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 927/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk927<F: Float>(t16004: F, t4726: F, t26: F, t16017: F, t1659: F, t4716: F, t6771: F, t1648: F, t4652: F, t6817: F, t16026: F, t16022: F, t5744: F, t657: F, t695: F, t311: F, t3841: F) -> (F, F, F, F, F, F, F, F) {
    let t16412 = t4726 * t16004;
    let t16413 = t26 * t16412;
    let t16415 = t1659 * t16017;
    let t16416 = t26 * t16415;
    let t16418 = t4716 * t6771;
    let t16419 = t16418 * t1648;
    let t16421 = t6817 * t4652;
    let t16423 = t1659 * t16026;
    let t16424 = t26 * t16423;
    let t16426 = t1659 * t16022;
    let t16427 = t5744 * t16426;
    let t16430 = t657 * t695;
    let t16432 = t311 * t3841 * t16430;
    (t16413, t16416, t16419, t16421, t16424, t16427, t16430, t16432)
}
