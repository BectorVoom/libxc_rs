//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 357/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk357(t1136: f64, t1137: f64, t1086: f64, t1092: f64) -> (f64, f64, f64) {
    let t1138 = t1136 * t1137;
    let t1141 = 0.92708333333333333333e-2_f64 * t1086;
    let t1143 = -t1141 + 0.92708333333333333333e-2_f64 * t1092;
    (t1138, t1141, t1143)
}
