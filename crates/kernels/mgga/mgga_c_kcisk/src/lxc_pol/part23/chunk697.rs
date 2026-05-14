//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 697/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk697<F: Float>(t1320: F, t6211: F, t1310: F, t1309: F, t2164: F, t2170: F, t3935: F, t3966: F, t3970: F, t6172: F, t6176: F, t6180: F, t6184: F, t6189: F, t6197: F, t6201: F, t6207: F) -> (F, F, F) {
    let t6212 = t1320 * t6211;
    let t6213 = t1310 * t6212;
    let t6216 = -0.47975436576472845901e-1 * t3970 * t2164 + 0.59969295720591057377e-2 * t6172 + 0.23987718288236422951e-1 * t3935 * t6176 - 0.17990788716177317213e-1 * t3935 * t6180 - 0.35981577432354634426e-1 * t3935 * t6184 + 0.35981577432354634426e-1 * t1309 * t6189 - 0.5397236614853195164e-1 * t3966 * t2170 + 0.14392630972941853771e0 * t3970 * t2170 - 0.17990788716177317213e-1 * t6197 - 0.17990788716177317213e-1 * t3935 * t6201 + 0.10794473229706390328e0 * t1309 * t6207 - 0.5397236614853195164e-1 * t1309 * t6213;
    (t6212, t6213, t6216)
}
