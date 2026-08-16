//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 988/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk988(t1264: f64, t525: f64, t33428: f64, t615: f64, t8396: f64, t862: f64, t7898: f64, t315: f64, t323: f64, t8993: f64, t2149: f64, t29997: f64, t31912: f64, t31916: f64, t31918: f64, t31922: f64, t31926: f64, t31928: f64, t31937: f64, t7893: f64, t7912: f64, t7931: f64, t7932: f64, t8400: f64, t8402: f64, t8415: f64, t9003: f64) -> (f64, f64) {
    let t33561 = t525 * t1264;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    let t33575 = t33574 * t7898;
    let t33586 = 0.13170898365871023197e1_f64 * t315 * t8993 * t323;
    let t33588 = -0.8673628188205199462e0_f64 * t7931 * t7932 * t33561 - 0.17347256376410398924e1_f64 * t31912 + 0.17347256376410398924e1_f64 * t33566 * t2149 + 0.17347256376410398924e1_f64 * t7912 * t8415 - 0.13170898365871023197e1_f64 * t31916 - 0.65854491829355115987e0_f64 * t31918 - 0.34694512752820797848e1_f64 * t31922 + 0.34694512752820797848e1_f64 * t33575 + 0.17347256376410398924e1_f64 * t31926 - 0.8673628188205199462e0_f64 * t9003 * t7893 + 0.13170898365871023197e1_f64 * t31928 + 0.8673628188205199462e0_f64 * t8400 * t29997 * t8402 - t33586 - 0.17347256376410398924e1_f64 * t31937;
    (t33566, t33588)
}
