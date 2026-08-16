//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 445/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk445(t1450: f64, t3502: f64, t1340: f64, t1411: f64, t1404: f64, t1413: f64, t1441: f64, t1500: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3503 = t1450 * t3502;
    let t3504 = t1340 * t3503;
    let t3505 = t1411 * t3504;
    let t3507 = t1404 * t1413;
    let t3508 = t3507 * sigma0;
    let t3509 = t3508 * t1441;
    let t3510 = t1411 * t3509;
    let t3512 = t1500 * sigma0;
    (t3503, t3504, t3505, t3507, t3508, t3509, t3510, t3512)
}
