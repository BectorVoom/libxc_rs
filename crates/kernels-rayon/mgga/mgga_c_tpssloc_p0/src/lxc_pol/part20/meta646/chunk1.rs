//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2374/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2374(t10655: f64, t14392: f64, t14396: f64, t42023: f64, t2792: f64, t2836: f64, t4396: f64, t10661: f64, t14388: f64, t2793: f64, t10696: f64, t1557: f64) -> (f64, f64, f64, f64, f64) {
    let t48736 = 0.48245938496077605201e2_f64 * t10655 * t14392;
    let t48738 = 0.1551780387578202009e4_f64 * t42023 * t14396;
    let t48741 = 6.0_f64 * t2792 * t4396 * t2836;
    let t48744 = 0.28947563097646563121e3_f64 * t10661 * t14388 * t2793;
    let t48747 = 2.0_f64 * t2792 * t1557 * t10696;
    (t48736, t48738, t48741, t48744, t48747)
}
