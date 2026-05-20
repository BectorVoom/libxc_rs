//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3071/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3071<F: Float>(t448: F, t81218: F, t81250: F, t300: F, t1196: F, t16988: F, t20895: F, t20397: F, t5192: F, t24488: F, t3531: F, t20537: F, t5197: F) -> (F, F, F, F, F, F) {
    let t81252 = (t81218 + t81250) * t448;
    let t81254 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t81252;
    let t81257 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t20895 * t16988;
    let t81259 = F::cast_from(0.10389515463408878255e3_f64) * t5192 * t20397;
    let t81261 = F::cast_from(0.35089341735807877242e1_f64) * t3531 * t24488;
    let t81264 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t5197 * t20537;
    (t81252, t81254, t81257, t81259, t81261, t81264)
}
