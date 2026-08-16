//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 665/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk665(t157: f64, t5615: f64, t407: f64, t495: f64, t944: f64, t506: f64, t1410: f64, t1016: f64, t469: f64, t922: f64, t104: f64, t566: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5616 = t5615 * t157;
    let t5720 = t407 * t495;
    let t5746 = t944 * t495;
    let t5752 = t944 * t506;
    let t6263 = t944 * t1410;
    let t6337 = t1016 * t506;
    let t7288 = t469 * t922;
    let t7297 = t566 * t95 * t104;
    (t5616, t5720, t5746, t5752, t6263, t6337, t7288, t7297)
}
