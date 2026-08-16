//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1870/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1870(t1892: f64, t786: f64, t25877: f64, t14224: f64, t689: f64, t25304: f64, t27883: f64, t25898: f64, t2453: f64, t1955: f64, t27836: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    let t97705 = t14224 * t689;
    let t97799 = t25304 * t27883;
    let t97802 = t97699 * t25898;
    let t97916 = t2453 * t27883;
    let t97933 = t1955 * t27836 * t4075;
    (t97700, t97705, t97799, t97802, t97916, t97933)
}
