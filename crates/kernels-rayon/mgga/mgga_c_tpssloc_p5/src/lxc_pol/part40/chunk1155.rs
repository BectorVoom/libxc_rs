//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1155/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1155(t11282: f64, t6068: f64, t11285: f64, t1155: f64, t1164: f64, t11292: f64, t4883: f64, t15218: f64, t4882: f64, t1190: f64, t6238: f64, t1743: f64, t4965: f64) -> (f64, f64, f64, f64, f64) {
    let t18274 = t11282 * t6068;
    let t18275 = t11285 * t1155;
    let t18276 = t18274 * t18275;
    let t18278 = 0.10254018858216406658e4_f64 * t1164 * t18276;
    let t18279 = t11292 * t6068;
    let t18280 = t18279 * t4883;
    let t18282 = 0.10389515463408878255e3_f64 * t1164 * t18280;
    let t18283 = t4882 * t15218;
    let t18285 = 0.34631718211362927518e2_f64 * t1164 * t18283;
    let t18287 = t1190 * t6238;
    let t18297 = t4965 * t1743;
    (t18278, t18282, t18285, t18287, t18297)
}
