//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1035/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1035(t2290: f64, t7610: f64, t1198: f64, t1426: f64, t2297: f64, t598: f64, t30374: f64, t8477: f64, t1181: f64, t4555: f64, t599: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t34435 = t7610 * t2290;
    let t34446 = t598 * t1426 * t1198 * t2297;
    let t34449 = t30374 * t8477;
    let t34453 = t7493 * t1181 * t599 * t4555;
    (t34435, t34446, t34449, t34453)
}
