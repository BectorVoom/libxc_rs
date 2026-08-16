//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1039/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1039(t1569: f64, t7605: f64, t13364: f64, t31115: f64, t35633: f64, t1526: f64, t2020: f64, t2016: f64, t8747: f64, t31879: f64, t7637: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36372 = t7605 * t1569;
    let t36377 = t31115 * t13364 * t35633;
    let t36380 = t2020 * t1526;
    let t36382 = t2016 * t8747;
    let t36385 = 0.17149607247227894789e-2_f64 * t31879;
    let t36386 = t7637 * t8571;
    (t36372, t36377, t36380, t36382, t36385, t36386)
}
