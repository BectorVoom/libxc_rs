//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 689/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk689(t4292: f64, t508: f64, t1843: f64, t670: f64, t2616: f64, t2524: f64, t1534: f64, t72: f64, t757: f64, t1469: f64, t750: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4293 = t508 * t4292;
    let t4297 = t1843 * t670;
    let t4300 = 4.0_f64 * t2616;
    let t4301 = 0.5848223622634646207e0_f64 * t2524;
    let t4302 = t1534 * t72;
    let t4303 = t4302 * t757;
    let t4304 = 0.18311447306006545054e-3_f64 * t4303;
    let t4305 = t750 * t1469;
    let t4306 = t706 * t4305;
    (t4293, t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306)
}
