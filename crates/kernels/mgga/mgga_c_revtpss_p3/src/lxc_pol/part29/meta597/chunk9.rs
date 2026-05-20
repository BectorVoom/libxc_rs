//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2024/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2024<F: Float>(t10867: F, t2061: F, t14481: F, t2062: F, t2782: F, t26519: F, t99257: F, t28341: F, t786: F, t789: F, t10073: F, t1579: F, t2066: F, t25390: F) -> (F, F, F, F, F) {
    let t103452 = t10867 * t2061;
    let t103462 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t2062 * t14481;
    let t103463 = t99257 * t26519;
    let t103467 = F::cast_from(0.19514881078765566038e-1_f64) * t786 * t28341 * t789;
    let t103471 = t10073 * t25390 * t2066 * t1579;
    (t103452, t103462, t103463, t103467, t103471)
}
