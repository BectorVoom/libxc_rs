//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 895/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk895(t2375: f64, t3684: f64, t1294: f64, t2371: f64, t2528: f64, t1284: f64, t172: f64, t763: f64, t2535: f64, t184: f64, t3681: f64, t17: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3686 = 0.10843581300301739842e-1_f64 * t3684 * t2375;
    let t3688 = 0.11696447245269292414e1_f64 * t1294 * t2371;
    let t3690 = 0.17315859105681463759e2_f64 * t1294 * t2528;
    let t3691 = t1284 * t172;
    let t3692 = t3691 * t763;
    let t3693 = 0.11696447245269292414e1_f64 * t3692;
    let t3695 = 0.5848223622634646207e0_f64 * t1294 * t2535;
    let t3696 = t3681 * t184;
    let t3697 = t17 * t3696;
    (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697)
}
