//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 327/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk327(t1344: f64, t1408: f64, t1443: f64, t1448: f64, t1453: f64, t1553: f64, t1557: f64, t1598: f64, t1601: f64, t548: f64, t554: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t1607 = t1553 * t548 - 0.193e0_f64 * t1557 * t1598 + t1601 + 0.11607361111111111111e-2_f64 * t1344 + 0.17411041666666666666e-2_f64 * t1408 - 0.17411041666666666666e-2_f64 * t1443 - 0.46429444444444444443e-2_f64 * t1448 + 0.11607361111111111111e-2_f64 * t1453;
    let t1609 = t554 * t554;
    let t1610 = 1.0_f64 / t1609;
    let t1611 = t551 * t1610;
    (t1607, t1609, t1610, t1611)
}
