//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 715/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk715<F: Float>(t2227: F, t4616: F, t35861: F, t36156: F, t36173: F, t36200: F, t36204: F, t36034: F, t35496: F, t35565: F, t35607: F, t35611: F, t35616: F, t35618: F, t35621: F, t35696: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37423 = t4616 * t2227;
    let t37439 = 0.13659505348792789029e1 * t35861;
    let t37536 = 0.60578599471980134109e-2 * t36156;
    let t37544 = 0.13798458768617697214e-2 * t36173;
    let t37558 = 0.45531684495975963429e0 * t36200;
    let t37560 = 0.10371105912972302781e0 * t36204;
    let t37584 = 0.31113317738916908344e0 * t36034;
    let t37731 = 0.12649025447177706166e-6 * t35496;
    let t37768 = 0.487802396665200453e-2 * t35565;
    let t37786 = 0.91462949374725084936e-3 * t35607;
    let t37787 = 0.487802396665200453e-2 * t35611;
    let t37788 = 0.11709622077411463733e-2 * t35616;
    let t37789 = 0.18292589874945016987e-2 * t35618;
    let t37790 = 0.26021382394247697185e-3 * t35621;
    let t37815 = 0.89430439388620083049e-2 * t35696;
    (t37423, t37439, t37536, t37544, t37558, t37560, t37584, t37731, t37768, t37786, t37787, t37788, t37789, t37790, t37815)
}
