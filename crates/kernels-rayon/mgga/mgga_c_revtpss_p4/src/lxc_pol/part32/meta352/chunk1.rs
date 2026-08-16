//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1290/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1290(t10542: f64, t4500: f64, t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t4496: f64) -> (f64, f64, f64, f64) {
    let t14518 = 0.19514881078765566038e-1_f64 * t10542 * t4500;
    let t14519 = t4424 * t72;
    let t14520 = t14519 * t686;
    let t14522 = 0.19514881078765566038e-1_f64 * t2798 * t14520;
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    (t14518, t14522, t14525, t14533)
}
