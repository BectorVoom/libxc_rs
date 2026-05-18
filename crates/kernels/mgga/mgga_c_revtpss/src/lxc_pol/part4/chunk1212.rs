//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1212/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1212<F: Float>(t10513: F, t11026: F, t11030: F, t11037: F, t11040: F, t11045: F, t11051: F, t15047: F, t15050: F, t15054: F, t15062: F, t15063: F, t1580: F, t213: F, t257: F, t2772: F, t4474: F) -> F {
    let t15069 = t15047 + t15050 - F::new(0.65854491829355115987e0) * t10513 * t1580 - F::new(0.54878743191129263322e-2) * t11026 + F::new(0.65854491829355115987e0) * t213 * t15054 * t257 - F::new(0.13009920719177044025e-2) * t11030 - F::new(0.10975748638225852664e-1) * t11037 + t15062 + F::new(0.73171657588172351096e-2) * t15063 - t11040 - F::new(0.19514881078765566038e-1) * t11045 - F::new(0.9757440539382783019e-2) * t11051 + F::new(0.13170898365871023197e1) * t4474 * t2772;
    t15069
}
