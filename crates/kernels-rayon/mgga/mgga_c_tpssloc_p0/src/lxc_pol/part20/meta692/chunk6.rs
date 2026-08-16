//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2641/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2641(t15908: f64, t9467: f64, t9882: f64, t118: f64, t2375: f64, t5151: f64, t16169: f64, t2663: f64, t1388: f64, t3734: f64, t15892: f64, t2371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53777 = t15908 * t9467;
    let t53778 = 0.21687162600603479684e-1_f64 * t53777;
    let t53779 = t15908 * t9882;
    let t53780 = 0.32530743900905219526e-1_f64 * t53779;
    let t53782 = t5151 * t118 * t2375;
    let t53783 = 0.32530743900905219526e-1_f64 * t53782;
    let t53787 = t16169 * t2663;
    let t53788 = 0.73245789224026180216e-3_f64 * t53787;
    let t53789 = t1388 * t3734;
    let t53796 = t15892 * t2371;
    (t53778, t53780, t53783, t53788, t53789, t53796)
}
