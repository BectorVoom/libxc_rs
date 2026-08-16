//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2472/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2472(t11094: f64, t3213: f64, t4696: f64, t4700: f64, t48734: f64, t48736: f64, t48738: f64, t48741: f64, t48744: f64, t48747: f64, t48750: f64, t48753: f64, t48755: f64, t48759: f64) -> f64 {
    let t50755 = 6.0_f64 * t11094 * t3213 * t4696 * t4700 - t48734 + t48736 + t48738 - t48741 - t48744 - t48747 - t48750 - t48753 - t48755 + t48759;
    t50755
}
