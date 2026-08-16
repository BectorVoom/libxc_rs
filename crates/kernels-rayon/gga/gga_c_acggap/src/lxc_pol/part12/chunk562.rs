//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 562/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk562(t323: f64, t3930: f64, t868: f64, t880: f64, t180: f64, t3054: f64, t865: f64, t191: f64, t813: f64, t301: f64, t467: f64, t4: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3932 = 0.19756347548806534796e1_f64 * t3930 * t323;
    let t3935 = t868 * t880;
    let t3937 = t3054 * t180;
    let t3939 = 0.39512695097613069591e1_f64 * t3937 * t865;
    let t3952 = 1.0_f64 / t813 / t191;
    let t3984 = t467 * t301;
    let t3992 = t483 * t4;
    (t3932, t3935, t3939, t3952, t3984, t3992)
}
