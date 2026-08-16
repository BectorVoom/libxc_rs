//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 993/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk993(t29997: f64, t7963: f64, t9029: f64, t524: f64, t9033: f64, t406: f64, t463: f64, t944: f64, t4241: f64, t7942: f64, t7884: f64, t8396: f64) -> (f64, f64, f64, f64, f64) {
    let t33672 = 0.17347256376410398924e1_f64 * t7963 * t29997 * t9029;
    let t33673 = t9033 * t524;
    let t33675 = t944 * t463 * t406;
    let t33681 = 0.34694512752820797848e1_f64 * t7942 * t33673 * t4241;
    let t33682 = t7884 * t8396;
    (t33672, t33673, t33675, t33681, t33682)
}
