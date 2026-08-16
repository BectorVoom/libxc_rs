//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1003/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1003(t3229: f64, t797: f64, t3275: f64, t3276: f64, t10945: f64, t10948: f64, t10957: f64, t10965: f64, t10970: f64, t10974: f64, t10983: f64, t10991: f64, t10996: f64, t11616: f64) -> (f64, f64, f64) {
    let t12428 = t797 * t3229;
    let t12430 = t3275 * t3276 * t12428;
    let t12431 = 5.0_f64 / 16.0_f64 * t12430;
    let t12432 = t10945 + t10948 + t10957 - t10965 + t10970 + t10974 - t10983 - 0.81300399444200075504e-3_f64 * t11616 + t10991 + t10996 + t12431;
    (t12428, t12431, t12432)
}
