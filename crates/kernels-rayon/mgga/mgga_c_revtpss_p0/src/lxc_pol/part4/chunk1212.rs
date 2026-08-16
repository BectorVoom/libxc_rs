//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1212/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1212(t10513: f64, t11026: f64, t11030: f64, t11037: f64, t11040: f64, t11045: f64, t11051: f64, t15047: f64, t15050: f64, t15054: f64, t15062: f64, t15063: f64, t1580: f64, t213: f64, t257: f64, t2772: f64, t4474: f64) -> f64 {
    let t15069 = t15047 + t15050 - 0.65854491829355115987e0_f64 * t10513 * t1580 - 0.54878743191129263322e-2_f64 * t11026 + 0.65854491829355115987e0_f64 * t213 * t15054 * t257 - 0.13009920719177044025e-2_f64 * t11030 - 0.10975748638225852664e-1_f64 * t11037 + t15062 + 0.73171657588172351096e-2_f64 * t15063 - t11040 - 0.19514881078765566038e-1_f64 * t11045 - 0.9757440539382783019e-2_f64 * t11051 + 0.13170898365871023197e1_f64 * t4474 * t2772;
    t15069
}
