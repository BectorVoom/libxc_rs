//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 971/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk971<F: Float>(t7: F, t8734: F, t8944: F, t1325: F, t1382: F, t2159: F, t220: F, t2337: F, t291: F, t3294: F, t3448: F, t771: F, t8590: F, t861: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t8945 = t8734 + t8944;
    let t8949 = piecewise3::<F>(t9, F::cast_from(0.0_f64), t8590 * t291 / F::cast_from(2.0_f64) + t3294 * t861 + t1325 * t2337 / F::cast_from(2.0_f64) + t2159 * t1382 / F::cast_from(2.0_f64) + t771 * t3448 + t220 * t8945 / F::cast_from(2.0_f64));
    (t8945, t8949)
}
