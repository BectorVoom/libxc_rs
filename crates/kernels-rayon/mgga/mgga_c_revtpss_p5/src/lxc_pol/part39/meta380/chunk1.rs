//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1364/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1364(t1196: f64, t16682: f64, t12500: f64, t5205: f64, t1733: f64, t3385: f64, t3433: f64, t3302: f64, t5332: f64, t1214: f64, t5333: f64, t1716: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16684 = 0.11696447245269292414e1_f64 * t1196 * t16682;
    let t16685 = t5205 * t12500;
    let t16687 = 0.17315859105681463759e2_f64 * t1196 * t16685;
    let t16688 = t1733 * t3385;
    let t16690 = 6.0_f64 * t3433 * t16688;
    let t16695 = t5332 * t3302;
    let t16696 = t5333 * t1214;
    let t16697 = t16695 * t16696;
    let t16706 = t2435 * t1716;
    (t16684, t16687, t16690, t16695, t16696, t16697, t16706)
}
