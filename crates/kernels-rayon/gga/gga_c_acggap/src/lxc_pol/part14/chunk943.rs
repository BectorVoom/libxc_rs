//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 943/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk943(t23688: f64, t7932: f64, t7942: f64, t33428: f64, t615: f64, t8396: f64, t862: f64, t7898: f64, t315: f64, t323: f64, t8993: f64, t7908: f64, t8998: f64) -> (f64, f64, f64, f64, f64) {
    let t33557 = 0.17347256376410398924e1_f64 * t7942 * t7932 * t23688;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    let t33575 = t33574 * t7898;
    let t33586 = 0.13170898365871023197e1_f64 * t315 * t8993 * t323;
    let t33606 = 0.34694512752820797848e1_f64 * t8998 * t7908;
    (t33557, t33566, t33575, t33586, t33606)
}
