//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1249/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1249<F: Float>(t15513: F, t291: F, t11399: F, t15406: F, t15413: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15495: F, t1622: F, t2938: F, t2963: F, t2971: F, t2989: F, t4647: F, t4670: F) -> (F, F) {
    let t15515 = F::new(0.621814e-1) * t15513 * t291;
    let t15516 = F::new(1.0) * t4647 * t2963 + F::cast_from(0.32163958997385070134e2_f64) * t15406 * t2971 + F::new(1.0) * t11399 * t1622 + F::new(2.0) * t2938 * t4670 - F::cast_from(0.11696447245269292414e1_f64) * t15413 * t2989 - t15418 - t15420 - t15423 - t15425 - t15427 - t15477 - F::cast_from(0.19751673498613801407e-1_f64) * t15495 + t15515;
    (t15515, t15516)
}
