//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 937/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk937(t2140: f64, t33429: f64, t1614: f64, t7976: f64, t29988: f64, t557: f64, t2132: f64, t2138: f64, t2331: f64, t879: f64, t2147: f64, t2341: f64) -> (f64, f64, f64, f64, f64) {
    let t33431 = 0.17347256376410398924e1_f64 * t33429 * t2140;
    let t33435 = 0.13170898365871023197e1_f64 * t7976 * t1614;
    let t33437 = 0.13170898365871023197e1_f64 * t29988 * t557;
    let t33444 = t2138 * t2132 * t2331 * t879;
    let t33451 = t2138 * t2147 * t2341 * t879;
    (t33431, t33435, t33437, t33444, t33451)
}
