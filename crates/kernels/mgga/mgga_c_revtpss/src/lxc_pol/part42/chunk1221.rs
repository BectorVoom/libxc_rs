//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1221/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1221<F: Float>(t1179: F, t1188: F, t20382: F, t1196: F, t5192: F, t5202: F, t5207: F, t1189: F, t6555: F, t5181: F, t5197: F, t16988: F, t5205: F, t300: F, t6513: F, t1198: F) -> (F, F, F, F, F, F, F) {
    let t20384 = t1179 * t20382 * t1188;
    let t20386 = 0.5848223622634646207e0 * t1196 * t20384;
    let t20388 = 0.11696447245269292414e1 * t5192 * t5202;
    let t20390 = 0.34631718211362927517e2 * t5192 * t5207;
    let t20391 = t6555 * t1189;
    let t20393 = 0.35089341735807877242e1 * t1196 * t20391;
    let t20394 = t5197 * t5181;
    let t20396 = 0.23392894490538584828e1 * t1196 * t20394;
    let t20397 = t5205 * t16988;
    let t20399 = 0.34631718211362927518e2 * t1196 * t20397;
    let t20400 = t300 * t6513;
    let t20402 = 0.5848223622634646207e0 * t20400 * t1198;
    (t20386, t20388, t20390, t20393, t20396, t20399, t20402)
}
