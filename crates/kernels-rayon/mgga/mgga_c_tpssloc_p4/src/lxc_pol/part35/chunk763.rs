//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 763/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk763(t2157: f64, t3640: f64, t112: f64, t2169: f64, t33: f64, t3953: f64, t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1433: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7398 = t2157 * t3640;
    let t7423 = t2169 * t112;
    let t7428 = t3953 * t33;
    let t7431 = t79 * t1437;
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7445 = t71 * t1433;
    (t7398, t7423, t7428, t7431, t7432, t7435, t7445)
}
