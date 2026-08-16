//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1028/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1028(t12929: f64, t374: f64, t11364: f64, t11365: f64, t11367: f64, t11585: f64, t11589: f64, t11593: f64, t11604: f64, t12738: f64, t12741: f64, t12744: f64, t12748: f64) -> (f64, f64) {
    let t12930 = t12929 * t374;
    let t12939 = 0.1440846329149835838e-2_f64 * t11585 + t12738 - t12741 + 0.1440846329149835838e-2_f64 * t11589 - 0.20496175532535769482e-3_f64 * t11593 - t12744 - 0.60975299583150056624e-3_f64 * t11604 + t11364 - t11365 - t12748 + t11367;
    (t12930, t12939)
}
