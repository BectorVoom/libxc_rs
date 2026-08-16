//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 933/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk933(t3700: f64, t993: f64, t1015: f64, t173: f64, t1029: f64, t3670: f64, t3645: f64, t460: f64, t13079: f64, t3090: f64, t13223: f64, t3038: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14421 = 0.34013387707001991332e-1_f64 * t3700 * t993;
    let t14423 = 1.0_f64 / t1015 / t173;
    let t14429 = t3670 * t1029;
    let t14442 = t3645 * t460;
    let t14446 = t13079 * t3090;
    let t14459 = 0.15805078039045227836e2_f64 * t13223 * t3038;
    (t14421, t14423, t14429, t14442, t14446, t14459)
}
