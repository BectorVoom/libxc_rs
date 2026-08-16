//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 955/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk955(t10751: f64, t10808: f64, t10859: f64, t10911: f64, t797: f64, t1048: f64, t499: f64, t3347: f64, t498: f64) -> (f64, f64, f64, f64) {
    let t10913 = t10751 + t10808 + t10859 + t10911;
    let t10914 = t10913 * t797;
    let t10916 = t1048 * t499 * t10914;
    let t10917 = t10916 / 4.0_f64;
    let t10918 = t498 * t3347;
    (t10913, t10914, t10917, t10918)
}
