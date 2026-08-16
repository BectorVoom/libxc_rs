//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 848/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk848(t11787: f64, t219: f64, t2809: f64, t699: f64, t709: f64, t11805: f64, t31: f64, t4: f64, t35: f64, t595: f64, t88: f64, t11870: f64, t2792: f64, t286: f64, t690: f64) -> (f64, f64, f64, f64, f64) {
    let t11909 = 24.0_f64 * t2809 * t11787 * t219;
    let t11910 = t709 * t699;
    let t11914 = 0.11483599538271604938e-1_f64 * t4 * t11805 * t31;
    let t11916 = t35 * t595 * t88;
    let t11921 = 0.6233709278045326953e3_f64 * t286 * t2792 * t11870 * t690;
    (t11909, t11910, t11914, t11916, t11921)
}
