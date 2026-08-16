//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 559/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk559(t3339: f64, t3498: f64, t980: f64, t161: f64, t3230: f64, t3233: f64, t3397: f64, t3409: f64, t3332: f64, t3330: f64, t3444: f64, t3453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3601 = 4.0_f64 / 9.0_f64 * t3339;
    let t3613 = 6.5831116232644735_f64 * t980 * t3498;
    let t3614 = t161 * t3230;
    let t3616 = t161 * t3233;
    let t3629 = 2.1389894610184537_f64 * t3397;
    let t3632 = 9.625452574583042_f64 * t3409;
    let t3633 = 0.8533333333333334_f64 * t3332;
    let t3634 = 0.14222222222222222_f64 * t3339;
    let t3643 = 0.64_f64 * t3330;
    let t3650 = 9.625452574583042_f64 * t3444;
    let t3652 = 25.667873532221446_f64 * t3453;
    (t3601, t3613, t3614, t3616, t3629, t3632, t3633, t3634, t3643, t3650, t3652)
}
