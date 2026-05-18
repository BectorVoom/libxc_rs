//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 802/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk802<F: Float>(t2227: F, t4616: F, t35861: F, t36156: F, t36173: F, t36200: F, t36204: F, t36034: F, t275: F, t8202: F, t35496: F, t8048: F, t942: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37423 = t4616 * t2227;
    let t37439 = F::new(0.13659505348792789029e1) * t35861;
    let t37536 = F::new(0.60578599471980134109e-2) * t36156;
    let t37544 = F::new(0.13798458768617697214e-2) * t36173;
    let t37558 = F::new(0.45531684495975963429e0) * t36200;
    let t37560 = F::new(0.10371105912972302781e0) * t36204;
    let t37584 = F::new(0.31113317738916908344e0) * t36034;
    let t37720 = t275 * t8202;
    let t37731 = F::new(0.12649025447177706166e-6) * t35496;
    let t37764 = t942 * t8048;
    (t37423, t37439, t37536, t37544, t37558, t37560, t37584, t37720, t37731, t37764)
}
