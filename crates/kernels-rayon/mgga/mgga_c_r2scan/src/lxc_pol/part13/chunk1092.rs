//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1092/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1092(t2262: f64, t4176: f64, t3270: f64, t1053: f64, t10648: f64, t10993: f64, t6876: f64, t58: f64, t423: f64, t2315: f64, t597: f64, t10680: f64) -> (f64, f64, f64, f64, f64) {
    let t38288 = t4176 * t2262;
    let t38289 = t3270 * t38288;
    let t38297 = t10648 * t1053 * t6876 * t10993;
    let t38298 = 0.91462949374725084942e-3_f64 * t38297;
    let t38299 = t6876 * t58;
    let t38300 = t38299 * t423;
    let t38301 = t597 * t2315;
    let t38303 = t10680 * t38300 * t38301;
    (t38289, t38298, t38299, t38301, t38303)
}
