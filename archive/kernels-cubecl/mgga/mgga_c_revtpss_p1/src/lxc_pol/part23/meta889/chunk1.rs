//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2820/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2820<F: Float>(t14546: F, t18525: F, t39697: F, t39701: F, t39719: F, t51424: F, t51430: F, t51435: F, t51445: F, t51452: F, t62716: F, t62723: F, t76131: F) -> F {
    let t76275 = F::cast_from(0.34697458558045176418e-2_f64) * t62716 - F::cast_from(0.34697458558045176418e-2_f64) * t62723 - F::cast_from(0.39029762157531132076e-2_f64) * t51424 + t51430 + t51435 + F::cast_from(0.91069445034239308177e-1_f64) * t51445 + t39697 + F::cast_from(0.33133632253434461091e-3_f64) * t51452 - F::cast_from(0.19637199382202157274e-3_f64) * t39701 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t76131 * t18525 + F::cast_from(0.19637199382202157274e-3_f64) * t39719;
    t76275
}
