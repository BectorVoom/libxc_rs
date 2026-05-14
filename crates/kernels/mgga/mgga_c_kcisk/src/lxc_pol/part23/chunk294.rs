//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 294/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk294<F: Float>(t1294: F, t1301: F, t1307: F, t1309: F, t1315: F, t1324: F, t405: F) -> (F,) {
    let t1327 = 0.5397236614853195164e-1 * t1294 * t405 - 0.14392630972941853771e0 * t1301 * t405 + t1307 + 0.17990788716177317213e-1 * t1309 * t1315 - 0.5397236614853195164e-1 * t1309 * t1324;
    (t1327,)
}
