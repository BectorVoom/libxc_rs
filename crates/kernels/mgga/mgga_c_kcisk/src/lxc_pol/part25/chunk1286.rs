//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1286/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1286<F: Float>(t116304: F, t17004: F, t32936: F, t10463: F, t1791: F, t17353: F, t34026: F, t33031: F, t112247: F, t112249: F, t112256: F, t116281: F, t116285: F, t116289: F, t116293: F, t116298: F, t15909: F, t15921: F, t15930: F, t17717: F, t17722: F, t33056: F, t34016: F, t7234: F, t9664: F) -> (F, F, F) {
    let t116306 = t116304 * t17004 * t32936;
    let t116311 = t1791 * t10463;
    let t116320 = t17353 * t34026;
    let t116321 = t33031 * t116320;
    let t116323 = -0.22109259259259259258e-2 * t116281 - 0.41666666666666666668e-1 * t9664 * t116285 + t116289 + 0.69444444444444444446e-2 * t112247 + 0.49745833333333333332e-2 * t116293 + 0.26805555555555555556e-2 * t112249 + 0.13402777777777777778e-2 * t112256 - 0.33163888888888888888e-2 * t116298 - 0.46296296296296296297e-2 * t33031 * t7234 * t34016 * t15921 + 0.41666666666666666668e-1 * t33031 * t116306 + 0.24125e-1 * t33056 * t116306 - 0.10802469135802469136e-1 * t33031 * t17717 * t116311 * t15930 + 0.18518518518518518519e-1 * t33031 * t17722 * t34016 * t15909 + 0.23148148148148148148e-2 * t116321;
    (t116306, t116320, t116323)
}
