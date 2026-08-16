//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1277/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1277(t133: f64, t1891: f64, t6601: f64, t80953: f64, t46511: f64, t6605: f64, t815: f64, t22816: f64, t23104: f64, t80967: f64, t23097: f64, t232: f64, t46606: f64) -> (f64, f64, f64, f64) {
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = 0.69792532988666768264e-2_f64 * t81735;
    let t81738 = t6605 * t815 * t46511;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = 0.43737152435318756759e-3_f64 * t81742;
    let t81746 = t23097 * t815 * t46606 * t232;
    (t81736, t81738, t81743, t81746)
}
