//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1238/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1238(t28173: f64, t8698: f64, t102019: f64, t1936: f64, t111018: f64, t28653: f64, t7002: f64, t1518: f64, t7356: f64, t2051: f64, t4292: f64, t34251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t128326 = 3.0_f64 * t8698 * t28173;
    let t128331 = t102019 * t1936;
    let t128332 = t111018 * t1936;
    let t128333 = t28653 * t7002;
    let t128334 = t7356 * t1518;
    let t128335 = t128334 * t1936;
    let t128336 = t2051 * t4292;
    let t128337 = t128336 * t1936;
    let t128338 = t34251 * t7002;
    (t128326, t128331, t128332, t128333, t128334, t128335, t128336, t128337, t128338)
}
