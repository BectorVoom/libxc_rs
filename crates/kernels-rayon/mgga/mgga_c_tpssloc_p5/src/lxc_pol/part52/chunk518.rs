//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 518/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk518(t868: f64, t870: f64, t2369: f64, t2509: f64, t2512: f64, t761: f64, t172: f64, t753: f64, t763: f64, t2504: f64, t739: f64, t746: f64) -> (f64, f64, f64, f64, f64) {
    let t2523 = t868 * t870;
    let t2527 = t2509 * t2369;
    let t2528 = t2527 * t2512;
    let t2530 = 0.17315859105681463759e2_f64 * t761 * t2528;
    let t2531 = t753 * t172;
    let t2532 = t2531 * t763;
    let t2535 = t739 * t2504 * t746;
    (t2523, t2528, t2530, t2532, t2535)
}
