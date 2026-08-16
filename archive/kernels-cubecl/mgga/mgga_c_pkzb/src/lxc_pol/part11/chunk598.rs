//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 598/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk598<F: Float>(t3278: F, t942: F, t1246: F, t1256: F, t3247: F, t3255: F, t411: F, t415: F, t938: F, t952: F, t1259: F, t2464: F) -> (F, F, F) {
    let t3279 = t942 * t3278;
    let t3282 = F::cast_from(0.65854491829355115987e0_f64) * t3247 * t415 - F::cast_from(0.65854491829355115987e0_f64) * t1246 * t952 - F::cast_from(0.65854491829355115987e0_f64) * t938 * t1256 + F::cast_from(0.13170898365871023197e1_f64) * t411 * t3255 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t3279;
    let t3286 = t1259 * t2464;
    (t3279, t3282, t3286)
}
