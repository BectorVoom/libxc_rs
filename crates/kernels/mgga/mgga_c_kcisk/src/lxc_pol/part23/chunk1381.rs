//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1381/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1381<F: Float>(t1411: F, t2266: F, t33608: F, t3786: F, t109717: F, t33596: F, t26773: F, t32045: F, t3777: F, t109919: F, t110640: F, t110648: F, t114444: F, t114448: F, t114454: F, t114455: F, t114458: F, t114462: F, t114464: F, t32030: F, t33377: F, t33384: F) -> (F, F, F, F) {
    let t114470 = t1411 * t33608 * t2266 * t3786;
    let t114473 = t1411 * t109717 * t33596;
    let t114477 = t1411 * t32045 * t26773 * t3777;
    let t114479 = 0.33163888888888888888e-2 * t114444 - 0.26805555555555555556e-2 * t110640 + 0.88437037037037037034e-2 * t114448 + 0.15432098765432098766e-2 * t110648 + 0.40208333333333333335e-2 * t33377 * t32030 + t114454 - 0.55273148148148148147e-2 * t114455 - 0.24872916666666666666e-2 * t114458 - 0.58958024691358024689e-2 * t109919 + 0.24320185185185185185e-1 * t114462 - 0.55273148148148148147e-3 * t114464 + 0.10416666666666666667e-1 * t33384 * t32030 + 0.33163888888888888888e-2 * t114470 + 0.88437037037037037034e-2 * t114473 + 0.55273148148148148147e-3 * t114477;
    (t114470, t114473, t114477, t114479)
}
