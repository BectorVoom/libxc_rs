//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1420/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1420(t10309: f64, t2826: f64, t136: f64, t10195: f64, t2770: f64, t9288: f64) -> (f64, f64, f64, f64, f64) {
    let t10310 = t2826 * t10309;
    let t10311 = t136 * t10310;
    let t10313 = t2826 * t10195;
    let t10314 = t136 * t10313;
    let t10316 = t2770 * t9288;
    (t10310, t10311, t10313, t10314, t10316)
}
