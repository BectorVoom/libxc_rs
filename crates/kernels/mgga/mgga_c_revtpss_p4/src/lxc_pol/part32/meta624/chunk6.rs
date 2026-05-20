//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1974/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1974<F: Float>(t1358: F, t212: F, t30247: F, t689: F, t102241: F, t102246: F, t102249: F, t102253: F, t108225: F, t108371: F, t108502: F, t1882: F, t2103: F, t25921: F, t25930: F, t28855: F, t28888: F, t28911: F, t30262: F, t543: F, t7295: F, t7301: F, t8095: F, t96257: F, t96260: F, t96265: F, t98050: F) -> F {
    let t109488 = t689 * t212 * t30247 * t1358;
    let t109493 = t102241 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t28888 * t1882 * t543 - t96257 - F::cast_from(0.22849835011101738147e-2_f64) * t96260 - t102246 + F::cast_from(0.8673628188205199462e0_f64) * t108225 * t28855 + F::cast_from(0.17347256376410398924e1_f64) * t98050 * t8095 - F::cast_from(0.34270468708064099208e-1_f64) * t96265 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t30262 - F::cast_from(0.14634331517634470219e-1_f64) * t102249 - F::cast_from(0.4336814094102599731e0_f64) * t108371 * t2103 - F::cast_from(0.54878743191129263322e-2_f64) * t109488 + F::cast_from(0.17347256376410398924e1_f64) * t25930 * t28911 * t108502 + t102253;
    t109493
}
