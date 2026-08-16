//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 764/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk764(t15388: f64, t68538: f64, t3154: f64, t38638: f64, t15266: f64, t16156: f64, t21719: f64, t35155: f64, t9183: f64, t236: f64, t446: f64, t615: f64) -> (f64, f64, f64, f64, f64) {
    let t73814 = t68538 * t15388;
    let t73816 = t38638 * t3154;
    let t73817 = 0.19863479950205658386e-4_f64 * t73816;
    let t73819 = t16156 * t15266;
    let t73822 = t21719 * t35155 * t9183;
    let t73825 = t236 * t615 * t446;
    (t73814, t73817, t73819, t73822, t73825)
}
