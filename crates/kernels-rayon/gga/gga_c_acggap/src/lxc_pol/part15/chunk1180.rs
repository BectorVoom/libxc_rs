//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1180/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1180(t1089: f64, t4643: f64, t598: f64, t8564: f64, t2297: f64, t8791: f64, t13364: f64, t33952: f64, t2046: f64, t336: f64, t5506: f64, t579: f64) -> (f64, f64, f64, f64) {
    let t40436 = t598 * t1089 * t4643 * t8564;
    let t40440 = t2297 * t8791;
    let t40442 = t33952 * t13364 * t40440;
    let t40446 = t2046 * t336 * t579 * t5506;
    (t40436, t40440, t40442, t40446)
}
