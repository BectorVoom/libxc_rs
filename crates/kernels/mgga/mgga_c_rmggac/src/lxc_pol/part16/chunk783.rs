//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 783/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk783<F: Float>(t36034: F, t35496: F, t35565: F, t35607: F, t35611: F, t35616: F, t35618: F, t35621: F, t35696: F, t35698: F, t35702: F, t35712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37584 = F::cast_from(0.31113317738916908344e0_f64) * t36034;
    let t37731 = F::cast_from(0.12649025447177706166e-6_f64) * t35496;
    let t37768 = F::cast_from(0.487802396665200453e-2_f64) * t35565;
    let t37786 = F::cast_from(0.91462949374725084936e-3_f64) * t35607;
    let t37787 = F::cast_from(0.487802396665200453e-2_f64) * t35611;
    let t37788 = F::cast_from(0.11709622077411463733e-2_f64) * t35616;
    let t37789 = F::cast_from(0.18292589874945016987e-2_f64) * t35618;
    let t37790 = F::cast_from(0.26021382394247697185e-3_f64) * t35621;
    let t37815 = F::cast_from(0.89430439388620083049e-2_f64) * t35696;
    let t37816 = F::cast_from(0.487802396665200453e-2_f64) * t35698;
    let t37818 = F::cast_from(0.18292589874945016987e-2_f64) * t35702;
    let t37821 = F::cast_from(0.18292589874945016987e-2_f64) * t35712;
    (t37584, t37731, t37768, t37786, t37787, t37788, t37789, t37790, t37815, t37816, t37818, t37821)
}
