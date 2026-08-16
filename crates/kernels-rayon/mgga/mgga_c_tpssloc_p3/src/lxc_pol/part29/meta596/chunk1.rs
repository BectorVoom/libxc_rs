//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2027/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2027(t81716: f64, t23150: f64, t814: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64, t22816: f64, t23104: f64, t80967: f64, t6612: f64, t812: f64, t836: f64) -> (f64, f64, f64, f64, f64) {
    let t81717 = 0.98696044010893586188e-1_f64 * t81716;
    let t81718 = t814 * t23150;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = 0.69792532988666768264e-2_f64 * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = 0.43737152435318756759e-3_f64 * t81742;
    let t81749 = t812 * t6612 * t836;
    (t81717, t81718, t81736, t81743, t81749)
}
