//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 547/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk547(t2528: f64, t761: f64, t172: f64, t753: f64, t763: f64, t2504: f64, t739: f64, t746: f64) -> (f64, f64, f64, f64, f64) {
    let t2530 = 0.17315859105681463759e2_f64 * t761 * t2528;
    let t2531 = t753 * t172;
    let t2532 = t2531 * t763;
    let t2533 = 0.11696447245269292414e1_f64 * t2532;
    let t2535 = t739 * t2504 * t746;
    (t2530, t2531, t2532, t2533, t2535)
}
