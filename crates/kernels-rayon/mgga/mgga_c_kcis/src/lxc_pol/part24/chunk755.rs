//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 755/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk755(t358: f64, t9543: f64, t283: f64, t1135: f64, t9528: f64, t1018: f64, t86: f64, t9526: f64, t1024: f64, t1093: f64, t341: f64, t1004: f64, t110: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9545 = 1.0_f64 / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    let t9562 = t86 * t9526 * t1018;
    let t9563 = t9562 * t1024;
    let t9586 = t1093 * t1093;
    let t9587 = 1.0_f64 / t9586;
    let t9588 = t341 * t9587;
    let t9589 = t9588 * sigma0;
    let t9613 = t110 * t1004;
    (t9545, t9546, t9552, t9562, t9563, t9587, t9588, t9589, t9613)
}
