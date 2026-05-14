//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 716/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk716<F: Float>(t35698: F, t35702: F, t35712: F, t35716: F, t35728: F, t35776: F, t35781: F, t35786: F, t35798: F, t2265: F, t4036: F, t36330: F, t1347: F, t2244: F, t36504: F, t36527: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37816 = 0.487802396665200453e-2 * t35698;
    let t37818 = 0.18292589874945016987e-2 * t35702;
    let t37821 = 0.18292589874945016987e-2 * t35712;
    let t37822 = 0.26021382394247697185e-3 * t35716;
    let t37825 = 0.13010691197123848592e-3 * t35728;
    let t37848 = 0.30487649791575028312e-3 * t35776;
    let t37849 = 0.89430439388620083049e-2 * t35781;
    let t37850 = 0.3286404220903135089e-2 * t35786;
    let t37860 = 0.2439011983326002265e-2 * t35798;
    let t37866 = t4036 * t2265;
    let t37872 = 0.18292589874945016987e-2 * t36330;
    let t37904 = t1347 * t2244;
    let t37964 = 0.13659505348792789029e1 * t36504;
    let t37976 = 0.2439011983326002265e-2 * t36527;
    (t37816, t37818, t37821, t37822, t37825, t37848, t37849, t37850, t37860, t37866, t37872, t37904, t37964, t37976)
}
