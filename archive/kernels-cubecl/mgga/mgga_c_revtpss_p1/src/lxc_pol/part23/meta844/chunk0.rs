//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2722/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2722<F: Float>(t1261: F, t20981: F, t3172: F, t13033: F, t21188: F, t20985: F, t20820: F, t3704: F, t17720: F, t5381: F, t20810: F, t3711: F) -> (F, F, F, F, F, F) {
    let t70369 = t1261 * t3172 * t20981;
    let t70373 = t13033 * t21188;
    let t70376 = t1261 * t3172 * t20985;
    let t70378 = t20820 * t3704;
    let t70382 = t5381 * t17720;
    let t70394 = t3711 * t3172 * t20810;
    (t70369, t70373, t70376, t70378, t70382, t70394)
}
