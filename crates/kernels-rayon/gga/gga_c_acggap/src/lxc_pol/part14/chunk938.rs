//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 938/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk938(t872: f64, t9062: f64, t2347: f64, t30005: f64, t7990: f64, t8419: f64, t7987: f64, t8423: f64, t7306: f64, t8397: f64, t2331: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33459 = 0.13170898365871023197e1_f64 * t9062 * t872;
    let t33465 = t30005 * t2347;
    let t33468 = 0.17347256376410398924e1_f64 * t7990 * t8419;
    let t33475 = 0.17347256376410398924e1_f64 * t7987 * t8423;
    let t33488 = 0.34694512752820797848e1_f64 * t8397 * t7306;
    let t33489 = t394 * t2331;
    (t33459, t33465, t33468, t33475, t33488, t33489)
}
