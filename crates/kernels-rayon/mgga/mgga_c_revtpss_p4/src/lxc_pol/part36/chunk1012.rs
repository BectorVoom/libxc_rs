//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1012/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1012(t23481: f64, t2908: f64, t141: f64, t23485: f64, t930: f64, t4573: f64, t5825: f64, t2850: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t23492 = t2908 * t23481;
    let t23493 = t141 * t23492;
    let t23495 = t930 * t23485;
    let t23496 = t141 * t23495;
    let t23499 = t4573 * t5825;
    let t23500 = t2850 * t23499;
    let t23501 = t128 * t23500;
    (t23493, t23496, t23499, t23501)
}
