//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1478/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1478(t31398: f64, t31461: f64, t3: f64, t2198: f64, t670: f64, t1518: f64, t31234: f64, t4292: f64, t8342: f64, t116: f64, t8406: f64, t117: f64, t31451: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31463 = 2.0_f64 * t31398 + 2.0_f64 * t31461;
    let t31464 = t3 * t31463;
    let t31475 = param_d * t31463;
    let t31493 = t670 * t2198;
    let t31494 = t31493 * t1518;
    let t31497 = t31234 * t1518;
    let t31500 = t8342 * t4292;
    let t31505 = t116 * t8406;
    let t31506 = t31505 * t670;
    let t31509 = t117 * t31451;
    (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509)
}
