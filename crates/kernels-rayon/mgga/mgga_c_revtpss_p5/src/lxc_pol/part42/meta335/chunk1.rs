//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1135/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1135(t15008: f64, t689: f64, t213: f64, t4469: f64, t1580: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t4321: f64, t887: f64) -> (f64, f64, f64, f64, f64) {
    let t15010 = 0.10975748638225852664e-1_f64 * t689 * t15008;
    let t15011 = t213 * t4469;
    let t15014 = t2440 * t1580;
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15045 = t4321 * t887;
    (t15010, t15011, t15015, t15018, t15045)
}
