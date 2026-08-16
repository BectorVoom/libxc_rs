//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 817/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk817(t1474: f64, t67: f64, t758: f64, t2431: f64, t2532: f64, t2653: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4211 = t1474 * t67;
    let t4212 = t4211 * t758;
    let t4213 = 0.18311447306006545054e-3_f64 * t4212;
    let t4214 = 4.0_f64 * t2431;
    let t4215 = 0.5848223622634646207e0_f64 * t2532;
    let t4216 = 0.18311447306006545054e-3_f64 * t2653;
    let t4217 = t2417 - t2423 - t2426 - t4213 + t4214 + t2518 - t2530 - t4215 - t2537 + t2538 + t2665 - t4216 - t2486;
    (t4211, t4212, t4213, t4214, t4215, t4216, t4217)
}
