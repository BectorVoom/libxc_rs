//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 955/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk955(t1123: f64, t9528: f64, t1085: f64, t3225: f64, t329: f64, t64: f64, t358: f64, t283: f64, t1135: f64, t1018: f64, t86: f64, t9526: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9529 = t9528 * t1123;
    let t9531 = t1085 * t3225;
    let t9532 = t9531 * sigma0;
    let t9543 = t64 * t329;
    let t9545 = 1.0_f64 / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    let t9562 = t86 * t9526 * t1018;
    (t9529, t9531, t9532, t9545, t9546, t9552, t9562)
}
