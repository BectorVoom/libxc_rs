//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 263/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk263(t286: f64, t912: f64, t659: f64, t706: f64, t711: f64, t714: f64, t717: f64, t753: f64, t757: f64, t774: f64, t782: f64, t809: f64, t910: f64) -> (f64, f64) {
    let t913 = t286 * t912;
    let t914 = 0.11696447245269292414e1_f64 * t913;
    let t915 = t711 + t714 - t717 - t753 + t910 + t774 + t782 + t659 + t809 + t914 - t706 - t757;
    (t914, t915)
}
