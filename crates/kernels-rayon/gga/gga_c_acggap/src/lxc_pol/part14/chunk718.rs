//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 718/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk718(t409: f64, t7685: f64, t2100: f64, t7676: f64, t1988: f64, t2092: f64, t1459: f64, t7458: f64, t7486: f64, t1980: f64, t2117: f64, t377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7686 = t7685 * t409;
    let t7696 = t7676 * t2100;
    let t7697 = 0.18868855373762491241e-2_f64 * t7696;
    let t7698 = t1988 * t2092;
    let t7709 = t7458 * t1459 * t7486;
    let t7710 = t1980 * t7709;
    let t7712 = t377 * t2117;
    (t7686, t7697, t7698, t7709, t7710, t7712)
}
