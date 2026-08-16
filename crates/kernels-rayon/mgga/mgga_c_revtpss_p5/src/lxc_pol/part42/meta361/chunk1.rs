//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1177/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1177(t1247: f64, t17544: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64, t3708: f64, t5265: f64, t1260: f64, t5326: f64, t3704: f64, t5274: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17546 = 0.28582678745379824648e-3_f64 * t1247 * t17544;
    let t17547 = t3707 * t5292;
    let t17550 = t3617 * t12268;
    let t17556 = 0.28582678745379824648e-3_f64 * t3708 * t5265;
    let t17569 = t5326 * t1260;
    let t17593 = 0.28582678745379824648e-3_f64 * t5274 * t3704;
    (t17546, t17547, t17550, t17556, t17569, t17593)
}
