//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1149/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1149(t27394: f64, t7080: f64, t6176: f64, t2237: f64, t28397: f64, t28415: f64, t28424: f64, t28427: f64, t28544: f64, t29284: f64, t29300: f64, t29305: f64, t29308: f64, t29311: f64, t29314: f64, t29324: f64, t7898: f64, t7908: f64, t8144: f64, t8148: f64, t8159: f64) -> (f64, f64, f64) {
    let t29331 = t27394 * t7080;
    let t29332 = t6176 * t29331;
    let t29335 = 0.18550940104166666667e-3_f64 * t28397 * t8148 + 0.92754700520833333333e-4_f64 * t7898 * t29300 + 0.22109259259259259258e-2_f64 * t28415 - 0.88437037037037037034e-2_f64 * t29305 + 0.16581944444444444444e-2_f64 * t29308 - 0.55273148148148148147e-3_f64 * t29311 + 0.46336805555555555556e-3_f64 * t7908 * t29314 + 0.46336805555555555556e-3_f64 * t7908 * t29284 + 0.46336805555555555556e-3_f64 * t28424 + 0.61836467013888888889e-4_f64 * t28427 + 0.13901041666666666667e-2_f64 * t8144 * t8159 - 0.2782641015625e-3_f64 * t7898 * t29324 - 0.4946917361111111111e-3_f64 * t28544 * t8148 - 0.13901041666666666667e-2_f64 * t2237 * t29324 - 0.13901041666666666667e-2_f64 * t2237 * t29332;
    (t29331, t29332, t29335)
}
