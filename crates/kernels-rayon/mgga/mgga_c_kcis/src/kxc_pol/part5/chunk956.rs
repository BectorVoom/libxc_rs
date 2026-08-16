//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 956/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk956(t1024: f64, t9562: f64, t1093: f64, t341: f64, t1004: f64, t110: f64, t285: f64, t3030: f64, t961: f64, t273: f64, t3033: f64, t2985: f64, t926: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9563 = t9562 * t1024;
    let t9586 = t1093 * t1093;
    let t9587 = 1.0_f64 / t9586;
    let t9588 = t341 * t9587;
    let t9589 = t9588 * sigma0;
    let t9613 = t110 * t1004;
    let t9614 = t285 * t9613;
    let t9630 = 1.0_f64 / t3030 / t961;
    let t9634 = 1.0_f64 / t3033 / t273;
    let t9655 = t926 * t2985;
    (t9563, t9587, t9588, t9589, t9614, t9630, t9634, t9655)
}
