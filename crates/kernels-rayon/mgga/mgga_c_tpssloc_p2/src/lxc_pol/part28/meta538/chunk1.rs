//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1799/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1799(t133: f64, t1891: f64, t6601: f64, t80953: f64, t22816: f64, t23104: f64, t80967: f64, t6612: f64, t812: f64, t836: f64, t2649: f64, t2690: f64, t6619: f64) -> (f64, f64, f64, f64, f64) {
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81749 = t812 * t6612 * t836;
    let t81750 = t81749 * t2649;
    let t81763 = t812 * t6619 * t2690;
    (t81735, t81742, t81749, t81750, t81763)
}
