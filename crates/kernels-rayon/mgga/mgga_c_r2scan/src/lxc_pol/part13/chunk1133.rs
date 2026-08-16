//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1133/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1133(t10781: f64, t7373: f64, t10776: f64, t3308: f64, t7990: f64, t37674: f64, t37676: f64, t37681: f64, t37696: f64, t37700: f64, t39569: f64, t39572: f64, t39577: f64, t39580: f64, t39581: f64) -> f64 {
    let t39583 = t10781 * t7373;
    let t39586 = t10776 * t3308 * t7990;
    let t39590 = 0.21831846657716620896e-2_f64 * t39569 + 0.13099107994629972538e-1_f64 * t39572 - 0.69345773920434148506e0_f64 * t37674 + 0.23115257973478049502e0_f64 * t37676 - 0.48787202696913915093e-2_f64 * t37681 + 0.54878743191129263322e-1_f64 * t39577 - t39580 + 0.17336443480108537126e0_f64 * t39581 + 0.54878743191129263322e-1_f64 * t39583 + 0.43341108700271342816e-1_f64 * t39586 + 0.23287303101564395623e-1_f64 * t37696 + 0.11708928647259339622e0_f64 * t37700;
    t39590
}
