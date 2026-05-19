//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 281/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk281<F: Float>(t1320: F, t1322: F, t1310: F, t1294: F, t1301: F, t1307: F, t1309: F, t1315: F, t405: F, t408: F, t11: F, t139: F, t79: F) -> (F, F, F, F, F, F) {
    let t1323 = t1320 * t1322;
    let t1324 = t1310 * t1323;
    let t1327 = F::cast_from(0.5397236614853195164e-1_f64) * t1294 * t405 - F::cast_from(0.14392630972941853771e0_f64) * t1301 * t405 + t1307 + F::cast_from(0.17990788716177317213e-1_f64) * t1309 * t1315 - F::cast_from(0.5397236614853195164e-1_f64) * t1309 * t1324;
    let t1328 = F::new(1.0) / t408;
    let t1329 = t1327 * t1328;
    let t1333 = t139 * t11 * t79;
    (t1323, t1324, t1327, t1328, t1329, t1333)
}
