//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 436/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk436<F: Float>(t1121: F, t3417: F, t397: F, t1111: F, t1119: F, t1125: F, t119: F, t268: F, t275: F, t3042: F, t3392: F, t3399: F, t3406: F, t3413: F, t918: F) -> (F, F) {
    let t3419 = t397 * t1121 * t3417;
    let t3422 = F::cast_from(0.5397236614853195164e-1_f64) * t3392 * t119 * t275 - F::cast_from(0.25187104202648244098e0_f64) * t1111 * t918 * t275 - F::cast_from(0.10794473229706390328e0_f64) * t3399 * t1125 + F::cast_from(0.41978507004413740163e0_f64) * t268 * t3042 * t275 + F::cast_from(0.25187104202648244098e0_f64) * t3406 * t1125 + F::cast_from(0.10794473229706390328e0_f64) * t1119 * t3413 - F::cast_from(0.5397236614853195164e-1_f64) * t1119 * t3419;
    (t3419, t3422)
}
