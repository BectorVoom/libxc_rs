//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1133/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1133<F: Float>(t10781: F, t7373: F, t10776: F, t3308: F, t7990: F, t37674: F, t37676: F, t37681: F, t37696: F, t37700: F, t39569: F, t39572: F, t39577: F, t39580: F, t39581: F) -> F {
    let t39583 = t10781 * t7373;
    let t39586 = t10776 * t3308 * t7990;
    let t39590 = F::cast_from(0.21831846657716620896e-2_f64) * t39569 + F::cast_from(0.13099107994629972538e-1_f64) * t39572 - F::cast_from(0.69345773920434148506e0_f64) * t37674 + F::cast_from(0.23115257973478049502e0_f64) * t37676 - F::cast_from(0.48787202696913915093e-2_f64) * t37681 + F::cast_from(0.54878743191129263322e-1_f64) * t39577 - t39580 + F::cast_from(0.17336443480108537126e0_f64) * t39581 + F::cast_from(0.54878743191129263322e-1_f64) * t39583 + F::cast_from(0.43341108700271342816e-1_f64) * t39586 + F::cast_from(0.23287303101564395623e-1_f64) * t37696 + F::cast_from(0.11708928647259339622e0_f64) * t37700;
    t39590
}
