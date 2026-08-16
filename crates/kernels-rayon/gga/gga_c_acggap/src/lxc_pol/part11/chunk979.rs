//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 979/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk979(t32092: f64, t9030: f64, t30029: f64, t8407: f64, t1603: f64, t618: f64, t2137: f64, t2140: f64, t1614: f64, t7976: f64, t29988: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33414 = 0.17347256376410398924e1_f64 * t32092 * t9030;
    let t33416 = 0.17347256376410398924e1_f64 * t30029 * t8407;
    let t33428 = t1603 * t618;
    let t33429 = t2137 * t33428;
    let t33431 = 0.17347256376410398924e1_f64 * t33429 * t2140;
    let t33435 = 0.13170898365871023197e1_f64 * t7976 * t1614;
    let t33437 = 0.13170898365871023197e1_f64 * t29988 * t557;
    (t33414, t33416, t33428, t33431, t33435, t33437)
}
