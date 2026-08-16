//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 883/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk883<F: Float>(t109: F, t1444: F, t8138: F, t8127: F, t8128: F, t8137: F, t8223: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t8226 = t8138 * t1444;
    let t8230 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t8127 + t8128 * t8223 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8137 * t8226);
    (t8226, t8230)
}
