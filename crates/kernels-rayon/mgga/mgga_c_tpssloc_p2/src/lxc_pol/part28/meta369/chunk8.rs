//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1399/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1399(t14255: f64, t291: f64, t10629: f64, t1580: f64, t10632: f64, t2906: f64, t959: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64, t1561: f64, t2885: f64) -> (f64, f64, f64, f64, f64) {
    let t14257 = 0.621814e-1_f64 * t14255 * t291;
    let t14258 = t10629 * t1580;
    let t14259 = t10632 * t2906;
    let t14260 = t14258 * t14259;
    let t14262 = 0.10254018858216406658e4_f64 * t959 * t14260;
    let t14263 = t1573 * t2904;
    let t14266 = t4408 * t923;
    let t14271 = t1561 * t2885;
    (t14257, t14262, t14263, t14266, t14271)
}
