//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1093/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1093<F: Float>(t20160: F, t6506: F, t1580: F, t20149: F, t6489: F, t4397: F, t6473: F, t14878: F, t14886: F, t21932: F, t2322: F, t4381: F, t4388: F, t4393: F, t4408: F, t6459: F, t6490: F, t6507: F) -> (F,) {
    let t21937 = t20160 * t6506;
    let t21939 = 0.35981577432354634426e-1 * t1580 * t21937;
    let t21946 = t20149 * t6489;
    let t21947 = t1580 * t21946;
    let t21956 = 0.59969295720591057378e-2 * t4397 * t6473;
    let t21959 = 0.5397236614853195164e-1 * t1580 * t21932 - 0.28785261945883707542e0 * t4381 * t6507 + t21939 + 0.89953943580886586067e-2 * t14886 * t2322 + 0.35981577432354634426e-1 * t4397 * t6490 - 0.95950873152945691803e-1 * t4381 * t6490 - 0.41978507004413740163e-1 * t21947 + 0.89953943580886586067e-2 * t6459 * t4388 + 0.11993859144118211476e-1 * t6459 * t4393 - 0.47975436576472845902e-1 * t14878 * t2322 + t21956 - 0.17990788716177317213e-1 * t6459 * t4408;
    (t21959,)
}
