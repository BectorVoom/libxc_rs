//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 752/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk752(t2132: f64, t8422: f64, t2131: f64, t157: f64, t2331: f64, t406: f64, t2152: f64, t1410: f64, t609: f64, t2122: f64, t556: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8423 = t2132 * t8422;
    let t8424 = t2131 * t8423;
    let t8427 = t2331 * t406 * t157;
    let t8428 = t2152 * t8427;
    let t8432 = t609 * t1410 * t157;
    let t8433 = t2152 * t8432;
    let t8436 = t2122 * t556;
    let t8437 = t2147 * t8436;
    (t8423, t8424, t8428, t8433, t8436, t8437)
}
