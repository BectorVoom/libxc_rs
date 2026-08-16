//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 950/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk950(t10294: f64, t2403: f64, t909: f64, t2827: f64, t699: f64, t2830: f64, t2833: f64, t241: f64, t2978: f64, t2955: f64, t969: f64, t2967: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10295 = 20.0_f64 / 27.0_f64 * t10294;
    let t10296 = t2403 * t909;
    let t10298 = t699 * t2827;
    let t10300 = t699 * t2830;
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10331 = t2955 * t969;
    let t10333 = t964 * t2967;
    (t10295, t10296, t10298, t10300, t10302, t10304, t10331, t10333)
}
