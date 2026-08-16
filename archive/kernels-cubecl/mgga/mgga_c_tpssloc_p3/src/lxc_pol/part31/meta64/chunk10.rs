//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 414/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk414<F: Float>(t1174: F, t1195: F, t1198: F, t1203: F, t1213: F, t1218: F, t1224: F, t1227: F, t1232: F, t488: F) -> F {
    let t1235 = t1195 - t1174 * t1198 / F::cast_from(288.0_f64) + t1203 * t488 / F::cast_from(3072.0_f64) + t1213 * t1218 / F::cast_from(3072.0_f64) + t1224 - t1227 * t1232 / F::cast_from(4608.0_f64);
    t1235
}
