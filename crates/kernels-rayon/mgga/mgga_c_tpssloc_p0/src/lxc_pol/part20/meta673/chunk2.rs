//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2540/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540(t11419: f64, t1675: f64, t11424: f64, t15054: f64, t15057: f64, t44162: f64, t11185: f64, t15064: f64, t15068: f64, t43964: f64, t3264: f64, t3307: f64, t4782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51427 = t1675 * t11419;
    let t51437 = 6.0_f64 * t11424 * t15054;
    let t51439 = 0.28947563097646563121e3_f64 * t44162 * t15057;
    let t51441 = 0.48245938496077605201e2_f64 * t11185 * t15064;
    let t51443 = 0.1551780387578202009e4_f64 * t43964 * t15068;
    let t51446 = 6.0_f64 * t3264 * t4782 * t3307;
    (t51427, t51437, t51439, t51441, t51443, t51446)
}
