//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1024/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1024(t8543: f64, t8546: f64, t8549: f64, t8552: f64, t9341: f64, t9344: f64, t7430: f64, t7438: f64, t8090: f64, t8091: f64, t8093: f64, t8095: f64, t8096: f64, t8097: f64, t8098: f64) -> (f64, f64, f64, f64, f64) {
    let t42435 = 0.11974241701863808564e0_f64 * t8543;
    let t42436 = 0.35922725105591425692e0_f64 * t8546;
    let t42437 = 0.71845450211182851384e0_f64 * t8549;
    let t42438 = 0.17961362552795712846e0_f64 * t8552;
    let t42444 = 0.79828278012425390428e-1_f64 * t9341;
    let t42445 = 0.4726e1_f64 * t9344;
    let t42446 = t8090 + t8091 - 0.79453919800822633544e-4_f64 * t7430 + t8093 + 0.23836175940246790064e-3_f64 * t7438 + t42444 - t8095 - t42445 + t8096 + t8097 + t8098;
    (t42435, t42436, t42437, t42438, t42446)
}
