//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 705/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk705(t10054: f64, t2344: f64, t8659: f64, t2329: f64, t8365: f64, t209: f64, t605: f64, t615: f64, t236: f64, t1971: f64, t7453: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10055 = 0.17961362552795712846e0_f64 * t10054;
    let t10056 = t8659 * t2344;
    let t10057 = 0.20455996240684006296e-1_f64 * t10056;
    let t10058 = t8365 * t2329;
    let t10059 = 0.27274661654245341728e-1_f64 * t10058;
    let t10064 = t615 * t605 * t209;
    let t10065 = t236 * t10064;
    let t10066 = t1971 * t10065;
    let t10067 = t7453 * t10066;
    let t10068 = 0.1064114997332445985e-4_f64 * t10067;
    let t10070 = t618 * t605 * t209;
    (t10055, t10057, t10059, t10066, t10068, t10070)
}
