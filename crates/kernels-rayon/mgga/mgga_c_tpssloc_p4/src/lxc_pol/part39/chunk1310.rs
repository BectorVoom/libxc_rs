//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1310/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1310(t9365: f64, t99: f64, t64: f64, t2331: f64, t106: f64, t9364: f64, t111: f64, t3931: f64, t12723: f64, t112: f64, t16506: f64, t5363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35655 = t9365 * t99;
    let t35656 = t64 * t35655;
    let t35662 = t2331 * t99;
    let t35663 = t64 * t35662;
    let t45435 = 1.0_f64 / t9364 / t106;
    let t45560 = t3931 * t111;
    let t45632 = t12723 * t111;
    let t55341 = t16506 * t112;
    let t55353 = t5363 * t111;
    (t35656, t35663, t45435, t45560, t45632, t55341, t55353)
}
