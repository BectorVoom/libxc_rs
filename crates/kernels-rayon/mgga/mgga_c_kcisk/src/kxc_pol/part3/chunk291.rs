//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 291/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk291(t1354: f64, t1364: f64, t1346: f64, t429: f64, t431: f64, t446: f64, t301: f64, t41: f64) -> (f64, f64, f64, f64) {
    let t1365 = t1354 * t1364;
    let t1369 = 0.11955719325063177623e-1_f64 * t1346;
    let t1374 = 0.3513e-2_f64 * t429 * t446 * t431;
    let t1375 = t41 * t301;
    (t1365, t1369, t1374, t1375)
}
