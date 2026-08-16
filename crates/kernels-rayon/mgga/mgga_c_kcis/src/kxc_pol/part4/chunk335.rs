//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 335/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk335(t286: f64, t992: f64, t330: f64, t421: f64, t829: f64, t420: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1252 = t286 * t992;
    let t1253 = t421 * t330;
    let t1254 = t1253 * t829;
    let t1255 = t1252 * t1254;
    let t1258 = t420 * t420;
    let t1259 = 1.0_f64 / t1258;
    (t1252, t1253, t1254, t1255, t1258, t1259)
}
