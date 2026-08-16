//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2466/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2466(t3199: f64, t49649: f64, t10470: f64, t11045: f64, t381: f64, t1003: f64, t10359: f64, t11037: f64, t11043: f64, t11051: f64, t14574: f64, t14586: f64, t14595: f64, t14640: f64, t1610: f64, t1632: f64, t3200: f64, t3201: f64, t3202: f64, t3204: f64, t4615: f64, t4684: f64, t4689: f64, t49599: f64, t50509: f64, t50540: f64) -> f64 {
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50616 = -6.0_f64 * t14586 * t3200 * t4684 - 3.0_f64 * t14595 * t3200 * t4684 - 3.0_f64 * t3200 * t3201 * t50540 + 3.0_f64 * t49599 * t50509 * t50610 + 3.0_f64 * t1003 * t14640 + t10359 * t1632 - 6.0_f64 * t11037 * t14574 + t11043 * t1610 + 3.0_f64 * t11051 * t4689 - 3.0_f64 * t3202 * t50592 + 3.0_f64 * t3204 * t4615;
    t50616
}
