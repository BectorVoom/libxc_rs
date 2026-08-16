//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 131/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk131<F: Float>(t275: F, t291: F, t148: F, t154: F, t157: F, zeta_threshold: F) -> (F, F) {
    let t293 = F::cast_from(0.621814e-1_f64) * t275 * t291;
    let t294 = F::cast_from(2.0_f64) <= zeta_threshold;
    let t296 = piecewise3::<F>(t294, t148, F::cast_from(2.0_f64) * t154);
    let t297 = F::cast_from(0.0_f64) <= zeta_threshold;
    let t298 = piecewise3::<F>(t297, t148, F::cast_from(0.0_f64));
    let t300 = (t296 + t298 - F::cast_from(2.0_f64)) * t157;
    (t293, t300)
}
