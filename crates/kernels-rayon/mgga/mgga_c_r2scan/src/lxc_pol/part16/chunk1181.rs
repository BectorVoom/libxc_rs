//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1181/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1181(t10781: f64, t8839: f64, t10894: f64, t3072: f64, t10760: f64, t29283: f64, t6535: f64, t11793: f64, t2201: f64, t3613: f64, t12448: f64, t3336: f64) -> (f64, f64, f64, f64, f64) {
    let t43115 = t10781 * t8839;
    let t43117 = t10894 * t3072;
    let t43120 = t6535 * t10760 * t29283;
    let t43123 = t2201 * t3613 * t11793;
    let t43126 = t2201 * t3336 * t12448;
    (t43115, t43117, t43120, t43123, t43126)
}
