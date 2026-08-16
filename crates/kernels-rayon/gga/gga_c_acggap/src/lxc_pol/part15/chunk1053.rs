//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1053/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1053(t35290: f64, t35301: f64, t35315: f64, t35317: f64, t35348: f64, t35379: f64, t35384: f64, t35387: f64, t35390: f64, t35392: f64, t35394: f64, t35396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37476 = 0.42874018118069736972e-3_f64 * t35290;
    let t37479 = 0.31448092289604152068e-2_f64 * t35301;
    let t37484 = 0.12862205435420921092e-1_f64 * t35315;
    let t37485 = 0.34299214494455789578e-2_f64 * t35317;
    let t37498 = 0.14291339372689912324e-2_f64 * t35348;
    let t37519 = 0.62896184579208304138e-3_f64 * t35379;
    let t37522 = 0.61125e-1_f64 * t35384;
    let t37523 = t35387 / 4.0_f64;
    let t37524 = t35390 / 16.0_f64;
    let t37525 = 0.48018900292238105409e-1_f64 * t35392;
    let t37526 = 0.13719685797782315831e-1_f64 * t35394;
    let t37527 = 0.13719685797782315831e-1_f64 * t35396;
    (t37476, t37479, t37484, t37485, t37498, t37519, t37522, t37523, t37524, t37525, t37526, t37527)
}
