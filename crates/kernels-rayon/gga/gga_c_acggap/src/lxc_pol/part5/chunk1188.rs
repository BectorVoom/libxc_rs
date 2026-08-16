//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1188/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1188(t12586: f64, t6184: f64, t3382: f64, t6148: f64, t12589: f64, t5940: f64, t1008: f64, t5975: f64, t301: f64, t5506: f64, t1734: f64, t839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21607 = t12586 * t6184;
    let t21609 = t3382 * t6148;
    let t21611 = t12589 * t5940;
    let t21613 = t1008 * t5975;
    let t21615 = t5506 * t301;
    let t21620 = t1734 * t839;
    (t21607, t21609, t21611, t21613, t21615, t21620)
}
