//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1087/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1087(t119535: f64, t125343: f64, t125344: f64, t125345: f64, t125350: f64, t125355: f64, t125357: f64, t125359: f64, t1518: f64, t32162: f64, t32176: f64, t32178: f64, t33644: f64, t33646: f64, t4292: f64, t670: f64, t8564: f64) -> f64 {
    let t125361 = 2.0_f64 * t119535 * t1518 + 2.0_f64 * t125345 * t670 + 2.0_f64 * t125350 * t1518 + 2.0_f64 * t32162 * t4292 + t125343 + t125344 + 4.0_f64 * t125355 + 4.0_f64 * t125357 + 4.0_f64 * t125359 + t32176 + t32178 + t33644 + t33646 + t8564;
    t125361
}
