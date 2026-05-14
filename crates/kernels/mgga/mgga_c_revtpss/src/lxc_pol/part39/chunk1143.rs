//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1143/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1143<F: Float>(t4321: F, t887: F, t689: F, t4470: F, t786: F, t789: F, t14939: F, t225: F, t4534: F, t779: F, t2435: F, t4322: F, t10513: F, t11026: F, t11030: F, t11037: F, t11040: F, t11045: F, t11051: F, t1580: F, t213: F, t257: F, t2772: F, t4474: F) -> (F,) {
    let t15045 = t4321 * t887;
    let t15047 = 0.10975748638225852664e-1 * t689 * t15045;
    let t15048 = t786 * t4470;
    let t15050 = 0.19514881078765566038e-1 * t15048 * t789;
    let t15054 = t14939 * t225;
    let t15060 = t779 * t4534;
    let t15062 = 0.10975748638225852664e-1 * t689 * t15060;
    let t15063 = t2435 * t4322;
    let t15069 = t15047 + t15050 - 0.65854491829355115987e0 * t10513 * t1580 - 0.54878743191129263322e-2 * t11026 + 0.65854491829355115987e0 * t213 * t15054 * t257 - 0.13009920719177044025e-2 * t11030 - 0.10975748638225852664e-1 * t11037 + t15062 + 0.73171657588172351096e-2 * t15063 - t11040 - 0.19514881078765566038e-1 * t11045 - 0.9757440539382783019e-2 * t11051 + 0.13170898365871023197e1 * t4474 * t2772;
    (t15069,)
}
