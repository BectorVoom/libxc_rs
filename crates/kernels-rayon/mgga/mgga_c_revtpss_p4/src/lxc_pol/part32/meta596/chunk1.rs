//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1929/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1929(t10867: f64, t2061: f64, t14481: f64, t2062: f64, t2782: f64, t26519: f64, t99257: f64, t28341: f64, t786: f64, t789: f64, t10073: f64, t1579: f64, t2066: f64, t25390: f64) -> (f64, f64, f64, f64, f64) {
    let t103452 = t10867 * t2061;
    let t103462 = 0.21951497276451705328e-1_f64 * t2782 * t2062 * t14481;
    let t103463 = t99257 * t26519;
    let t103467 = 0.19514881078765566038e-1_f64 * t786 * t28341 * t789;
    let t103471 = t10073 * t25390 * t2066 * t1579;
    (t103452, t103462, t103463, t103467, t103471)
}
