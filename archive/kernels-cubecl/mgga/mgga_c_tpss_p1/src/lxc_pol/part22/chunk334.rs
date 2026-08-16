//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 334/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk334<F: Float>(t1072: F, t1080: F, t1081: F, t1089: F, t139: F, t215: F, t442: F, t441: F, t56: F, t924: F) -> (F, F, F, F, F) {
    let t1091 = t1072 * t1080 * t1081;
    let t1093 = F::cast_from(0.5848223622634646207e0_f64) * t1089 * t1091;
    let t1095 = t215 * t139 * t442;
    let t1097 = t441 * t1095 / F::cast_from(288.0_f64);
    let t1098 = t56 * t924;
    (t1091, t1093, t1095, t1097, t1098)
}
