//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 634/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk634<F: Float>(t1492: F, t2266: F, t486: F, t6311: F, t6314: F, t6319: F, t6324: F, t6326: F, t6329: F, t6334: F, t6337: F, t6341: F, t6345: F, t6347: F, t6349: F, t1501: F, t2279: F) -> (F, F, F, F) {
    let t6351 = t1492 * t2266;
    let t6352 = t486 * t6351;
    let t6354 = t6311 / 16.0 - t6314 / 24.0 - t6319 / 72.0 - t6324 / 576.0 - t6326 / 6.0 + t6329 / 192.0 + t6334 / 8.0 - t6337 / 48.0 + t6341 / 256.0 - t6345 / 256.0 + t6347 / 48.0 + t6349 / 24.0 - t6352 / 9.0;
    let t6355 = t1501 * t2279;
    (t6351, t6352, t6354, t6355)
}
