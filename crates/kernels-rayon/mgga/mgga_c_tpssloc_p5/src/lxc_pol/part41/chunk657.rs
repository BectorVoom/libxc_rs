//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 657/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk657(t4166: f64, t816: f64, t1500: f64, t838: f64, t842: f64, t242: f64, t2628: f64, t812: f64, t244: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4167 = t4166 * t816;
    let t4170 = t1500 * t838;
    let t4172 = t4166 * t842;
    let t4177 = t2628 * t242;
    let t4178 = t812 * t4177;
    let t4179 = t244 * t67;
    let t4180 = t4179 * t246;
    (t4167, t4170, t4172, t4177, t4178, t4180)
}
