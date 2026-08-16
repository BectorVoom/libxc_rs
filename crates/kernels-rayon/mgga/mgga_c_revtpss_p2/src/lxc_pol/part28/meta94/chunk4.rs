//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 601/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk601(t265: f64, t393: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t1962: f64, t207: f64, t198: f64, t892: f64, t1102: f64, t336: f64) -> (f64, f64, f64, f64) {
    let t394 = t265 < t393;
    let t1989 = 0.65854491829355115987e0_f64 * t342 * t1978 - 0.4336814094102599731e0_f64 * t1983 * t1986;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t1996 = piecewise3(t394, t198 * t336 * t1989 * t1102, t1995);
    (t1989, t1993, t1995, t1996)
}
