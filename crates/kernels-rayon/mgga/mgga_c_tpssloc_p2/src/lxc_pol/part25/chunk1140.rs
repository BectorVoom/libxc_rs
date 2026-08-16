//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1140/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1140(t1891: f64, t22816: f64, t23104: f64, t80967: f64, t23097: f64, t232: f64, t46606: f64, t815: f64, t6612: f64, t812: f64, t836: f64, t2649: f64) -> (f64, f64, f64) {
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81746 = t23097 * t815 * t46606 * t232;
    let t81749 = t812 * t6612 * t836;
    let t81750 = t81749 * t2649;
    (t81742, t81746, t81750)
}
