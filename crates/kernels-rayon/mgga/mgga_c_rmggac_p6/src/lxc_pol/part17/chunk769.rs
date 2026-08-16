//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 769/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk769(t36204: f64, t2067: f64, t25525: f64, t2078: f64, t3839: f64, t7834: f64, t838: f64, t25809: f64, t664: f64, t35583: f64, t793: f64, t35586: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36205 = 0.51855529564861513904e-1_f64 * t36204;
    let t36250 = t25525 * t2067;
    let t36254 = t3839 * t2078;
    let t36274 = t838 * t7834;
    let t36280 = t25809 * t664;
    let t36284 = t793 * t35583;
    let t36286 = t797 * t35586;
    (t36205, t36250, t36254, t36274, t36280, t36284, t36286)
}
