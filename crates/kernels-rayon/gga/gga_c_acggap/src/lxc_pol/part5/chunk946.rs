//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 946/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk946(t229: f64, t4059: f64, t1378: f64, t40: f64, t803: f64, t2824: f64, t483: f64, t1388: f64, t709: f64, t301: f64, t96: f64, t4068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14919 = t229 * t4059;
    let t14930 = t40 * t1378 * t803;
    let t14935 = t40 * t483 * t2824;
    let t14941 = t709 * t1388;
    let t14947 = t96 * t301;
    let t14957 = t229 * t4068;
    (t14919, t14930, t14935, t14941, t14947, t14957)
}
