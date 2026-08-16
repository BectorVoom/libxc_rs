//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2381/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2381(t13716: f64, t2932: f64, t10632: f64, t4471: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47705: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64) -> (f64, f64, f64) {
    let t48883 = t13716 * t2932;
    let t48890 = t4471 * t10632;
    let t48907 = -0.98587999999999999998e0_f64 * t48085 + 0.98587999999999999998e0_f64 * t48087 + 0.49293999999999999999e0_f64 * t48090 - 0.82156666666666666668e-1_f64 * t48092 - 0.88582716049382716048e0_f64 * t47681 + 0.35876000000000000001e1_f64 * t47686 - 0.59793333333333333333e0_f64 * t47691 - 0.59793333333333333333e0_f64 * t47695 - 0.19931111111111111111e0_f64 * t47699 - 0.53814000000000000001e1_f64 * t47703 + 0.79724444444444444445e0_f64 * t47705;
    (t48883, t48890, t48907)
}
