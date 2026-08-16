//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 762/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk762(t7586: f64, t8525: f64, t7585: f64, t2268: f64, t7839: f64, t2264: f64, t7433: f64, t1988: f64, t2310: f64, t1426: f64, t2297: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8526 = t7586 * t8525;
    let t8527 = t7585 * t8526;
    let t8529 = t7839 * t2268;
    let t8531 = t7433 * t2264;
    let t8533 = t1988 * t2310;
    let t8536 = t1426 * t429 * t2297;
    (t8526, t8527, t8529, t8531, t8533, t8536)
}
