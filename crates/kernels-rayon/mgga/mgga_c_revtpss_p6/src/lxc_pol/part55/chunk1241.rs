//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1241/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1241(t2322: f64, t34196: f64, t4254: f64, t1936: f64, t28586: f64, t651: f64, t28653: f64, t7003: f64, t128334: f64, t1937: f64, t128336: f64, t34251: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t128485 = 2.0_f64 * t2322 * t34196;
    let t128487 = 2.0_f64 * t4254 * t34196;
    let t128490 = 2.0_f64 * t651 * t28586 * t1936;
    let t128493 = 2.0_f64 * t28653 * t7003;
    let t128495 = 2.0_f64 * t128334 * t1937;
    let t128497 = 2.0_f64 * t128336 * t1937;
    let t128499 = 2.0_f64 * t34251 * t6993;
    (t128485, t128487, t128490, t128493, t128495, t128497, t128499)
}
