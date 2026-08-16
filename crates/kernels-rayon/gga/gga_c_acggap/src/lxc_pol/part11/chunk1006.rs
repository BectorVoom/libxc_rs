//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1006/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1006(t30037: f64, t2310: f64, t7610: f64, t2294: f64, t7780: f64, t137: f64, t4838: f64, t1083: f64, t1089: f64, t598: f64, t1524: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33872 = 0.68598428988911579156e-2_f64 * t30037;
    let t33874 = t7610 * t2310;
    let t33876 = t7780 * t2294;
    let t33878 = t137 * t4838;
    let t33881 = t598 * t1089 * t1083 * t33878;
    let t33883 = t355 * t1524;
    (t33872, t33874, t33876, t33878, t33881, t33883)
}
