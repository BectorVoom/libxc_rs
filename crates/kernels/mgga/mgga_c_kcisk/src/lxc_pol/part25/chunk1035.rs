//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1035/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1035<F: Float>(t4648: F, t7627: F, t1775: F, t4640: F, t5006: F, t12198: F, t15930: F, t12175: F, t12180: F, t12183: F, t12186: F, t12188: F, t12230: F, t18326: F, t18341: F, t18345: F, t18350: F, t18356: F, t18360: F, t18364: F, t18368: F, t18374: F, t18379: F, t18384: F, t2013: F, t5499: F, t7581: F) -> (F,) {
    let t18387 = t7627 * t4648;
    let t18388 = t1775 * t18387;
    let t18391 = t7627 * t4640;
    let t18392 = t5006 * t18391;
    let t18395 = t12198 * t15930;
    let t18396 = t5006 * t18395;
    let t18399 = -0.59969295720591057378e-2 * t12175 - 0.39979530480394038252e-2 * t12180 + 0.29984647860295528689e-2 * t12183 + 0.39979530480394038252e-2 * t12186 + 0.59969295720591057378e-2 * t12188 + 0.35981577432354634426e-1 * t18326 * t18341 + 0.5397236614853195164e-1 * t2013 * t18345 - 0.16191709844559585492e0 * t2013 * t18350 - 0.17990788716177317213e-1 * t7581 * t5499 - 0.19989765240197019126e-2 * t18356 - 0.17990788716177317213e-1 * t2013 * t18360 - 0.17990788716177317213e-1 * t2013 * t18364 + 0.17990788716177317213e-1 * t2013 * t18368 - 0.17990788716177317213e-1 * t12230 - 0.47975436576472845903e-1 * t2013 * t18374 - 0.89953943580886586067e-2 * t2013 * t18379 + 0.35981577432354634426e-1 * t2013 * t18384 - 0.89953943580886586067e-2 * t2013 * t18388 - 0.11993859144118211476e-1 * t2013 * t18392 - 0.71963154864709268855e-1 * t2013 * t18396;
    (t18399,)
}
