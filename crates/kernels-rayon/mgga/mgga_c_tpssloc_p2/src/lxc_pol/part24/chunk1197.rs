//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1197/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1197(t3034: f64, t371: f64, t1930: f64, t6741: f64, t3030: f64, t3127: f64, t363: f64, t1011: f64, t3040: f64, t3131: f64, t1014: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23510 = t23509 * t6741;
    let t23511 = t3030 * t3127;
    let t23512 = t23511 * t363;
    let t23513 = t3040 * t1011;
    let t23514 = t23513 * t3131;
    let t23515 = t23512 * t23514;
    let t23518 = t3030 * t1014;
    let t23519 = t23518 * t363;
    let t23520 = t23513 * t360;
    (t23508, t23509, t23510, t23511, t23514, t23515, t23518, t23519, t23520)
}
