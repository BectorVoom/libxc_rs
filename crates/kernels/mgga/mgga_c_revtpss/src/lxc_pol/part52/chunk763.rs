//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 763/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk763<F: Float>(t1904: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7279: F, t7288: F, t7291: F, t7295: F, t7911: F, t7917: F, t7921: F, t7926: F, t7930: F) -> F {
    let t7933 = -t7245 + t7248 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t7911 * t561 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t1904 + t7288 - t7291 - F::cast_from(0.4336814094102599731e0_f64) * t7917 * t2030 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7921 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7926 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t7930;
    t7933
}
