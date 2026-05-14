//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 283/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk283<F: Float>(t1320: F, t1322: F, t1310: F, t1294: F, t1301: F, t1307: F, t1309: F, t1315: F, t405: F) -> (F, F, F) {
    let t1323 = t1320 * t1322;
    let t1324 = t1310 * t1323;
    let t1327 = 0.5397236614853195164e-1 * t1294 * t405 - 0.14392630972941853771e0 * t1301 * t405 + t1307 + 0.17990788716177317213e-1 * t1309 * t1315 - 0.5397236614853195164e-1 * t1309 * t1324;
    (t1323, t1324, t1327)
}
