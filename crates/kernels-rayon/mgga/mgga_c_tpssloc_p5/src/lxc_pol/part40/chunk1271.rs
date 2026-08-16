//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1271/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1271(t5493: f64, t88: f64, t89: f64, t5456: f64, t576: f64, t2177: f64, t2281: f64, t2331: f64, t626: f64) -> (f64, f64, f64, f64, f64) {
    let t28007 = t88 * t5493;
    let t28030 = t89 * t5493;
    let t28893 = t576 * t5456;
    let t29894 = 11.0_f64 / 9.0_f64 * t2281 * t2177;
    let t29895 = t626 * t2331;
    (t28007, t28030, t28893, t29894, t29895)
}
