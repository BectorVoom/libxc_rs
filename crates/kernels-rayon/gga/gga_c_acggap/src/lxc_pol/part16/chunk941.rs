//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 941/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk941(t545: f64, t7923: f64, t621: f64, t2331: f64, t310: f64, t464: f64, t7984: f64, t8998: f64, t556: f64, t7932: f64) -> (f64, f64, f64, f64, f64) {
    let t33524 = t7923 * t545;
    let t33525 = t33524 * t621;
    let t33527 = t310 * t2331;
    let t33529 = 0.13170898365871023197e1_f64 * t33527 * t464;
    let t33533 = 0.17347256376410398924e1_f64 * t8998 * t7984;
    let t33535 = t7932 * t556;
    (t33525, t33527, t33529, t33533, t33535)
}
