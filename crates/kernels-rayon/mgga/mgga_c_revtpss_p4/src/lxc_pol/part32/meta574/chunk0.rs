//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1899/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1899(t1358: f64, t2439: f64, t785: f64, t8085: f64, t1364: f64, t28905: f64, t786: f64, t96187: f64, t97688: f64, t28791: f64, t689: f64, t25899: f64) -> (f64, f64, f64, f64, f64) {
    let t102139 = t2439 * t785 * t8085 * t1358;
    let t102143 = 0.19514881078765566038e-1_f64 * t786 * t28905 * t1364;
    let t102164 = 0.28912093960683998208e-1_f64 * t96187 * t97688;
    let t102165 = t28791 * t689;
    let t102167 = 0.25702851531048074406e-1_f64 * t25899 * t102165;
    (t102139, t102143, t102164, t102165, t102167)
}
