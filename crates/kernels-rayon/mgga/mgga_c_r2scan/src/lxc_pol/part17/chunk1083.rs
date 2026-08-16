//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1083/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1083(t1053: f64, t10648: f64, t10993: f64, t6876: f64, t58: f64, t423: f64, t2315: f64, t597: f64, t10680: f64, t10977: f64, t10981: f64, t37368: f64) -> (f64, f64, f64, f64, f64) {
    let t38297 = t10648 * t1053 * t6876 * t10993;
    let t38299 = t6876 * t58;
    let t38300 = t38299 * t423;
    let t38301 = t597 * t2315;
    let t38303 = t10680 * t38300 * t38301;
    let t38311 = t37368 * t10977 * t10981;
    (t38297, t38299, t38301, t38303, t38311)
}
