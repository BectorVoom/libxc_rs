//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 690/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk690<F: Float>(t1315: F, t1324: F, t2164: F, t3966: F, t3975: F, t3983: F, t3985: F, t3996: F, t4004: F, t405: F, t6149: F, t6152: F, t6155: F, t6157: F, t2163: F, t3973: F) -> (F, F) {
    let t6168 = 0.5397236614853195164e-1 * t6149 * t405 - 0.14392630972941853771e0 * t6152 * t405 + 0.17990788716177317213e-1 * t6155 + 0.17990788716177317213e-1 * t6157 * t1315 - 0.5397236614853195164e-1 * t6157 * t1324 + 0.17990788716177317213e-1 * t3996 - 0.47975436576472845901e-1 * t4004 - t3983 + 0.59969295720591057377e-2 * t3975 - 0.17990788716177317213e-1 * t3985 + 0.17990788716177317213e-1 * t3966 * t2164;
    let t6171 = t3973 * t2163;
    (t6168, t6171)
}
