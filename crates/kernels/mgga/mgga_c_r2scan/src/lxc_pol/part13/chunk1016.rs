//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1016/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1016<F: Float>(t37674: F, t37676: F, t37681: F, t37696: F, t37700: F, t39569: F, t39572: F, t39577: F, t39580: F, t39581: F, t39583: F, t39586: F, t1058: F, t1060: F, t2201: F, t7290: F) -> (F, F) {
    let t39590 = 0.21831846657716620896e-2 * t39569 + 0.13099107994629972538e-1 * t39572 - 0.69345773920434148506e0 * t37674 + 0.23115257973478049502e0 * t37676 - 0.48787202696913915093e-2 * t37681 + 0.54878743191129263322e-1 * t39577 - t39580 + 0.17336443480108537126e0 * t39581 + 0.54878743191129263322e-1 * t39583 + 0.43341108700271342816e-1 * t39586 + 0.23287303101564395623e-1 * t37696 + 0.11708928647259339622e0 * t37700;
    let t39599 = t2201 * t1058 * t1060 * t7290;
    (t39590, t39599)
}
