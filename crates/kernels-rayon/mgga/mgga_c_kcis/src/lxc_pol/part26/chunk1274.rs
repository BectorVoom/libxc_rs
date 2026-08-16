//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1274/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1274(t17308: f64, t8207: f64, t28570: f64, t48058: f64, t22714: f64, t7940: f64, t27491: f64, t7397: f64, t28778: f64, t28853: f64, t28713: f64, t6140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101837 = 2.0_f64 * t17308 * t8207;
    let t101839 = 12.0_f64 * t48058 * t28570;
    let t101840 = t7940 * t22714;
    let t101841 = t27491 * t7397;
    let t101849 = t28853 * t28778;
    let t101853 = t28713 * t6140;
    (t101837, t101839, t101840, t101841, t101849, t101853)
}
