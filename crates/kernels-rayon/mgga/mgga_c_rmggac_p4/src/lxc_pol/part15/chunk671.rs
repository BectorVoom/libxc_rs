//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 671/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk671(t9229: f64, t2347: f64, t570: f64, t262: f64, t7204: f64, t558: f64) -> (f64, f64, f64, f64, f64) {
    let t9672 = 0.5987120850931904282e-1_f64 * t9229;
    let t9704 = t2347 * t570;
    let t9705 = t262 * t9704;
    let t9706 = t7204 * t9705;
    let t9707 = 0.20455996240684006296e-1_f64 * t9706;
    let t9708 = t2347 * t558;
    (t9672, t9704, t9705, t9707, t9708)
}
