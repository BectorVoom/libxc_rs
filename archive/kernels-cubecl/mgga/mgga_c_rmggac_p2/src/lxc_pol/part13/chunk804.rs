//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 804/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk804<F: Float>(t35776: F, t35781: F, t35786: F, t35798: F, t2265: F, t4036: F, t36330: F, t1347: F, t2244: F, t4028: F, t702: F, t275: F, t8292: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37848 = F::cast_from(0.30487649791575028312e-3_f64) * t35776;
    let t37849 = F::cast_from(0.89430439388620083049e-2_f64) * t35781;
    let t37850 = F::cast_from(0.3286404220903135089e-2_f64) * t35786;
    let t37860 = F::cast_from(0.2439011983326002265e-2_f64) * t35798;
    let t37866 = t4036 * t2265;
    let t37872 = F::cast_from(0.18292589874945016987e-2_f64) * t36330;
    let t37904 = t1347 * t2244;
    let t37921 = t4028 * t702;
    let t37948 = t275 * t8292;
    (t37848, t37849, t37850, t37860, t37866, t37872, t37904, t37921, t37948)
}
