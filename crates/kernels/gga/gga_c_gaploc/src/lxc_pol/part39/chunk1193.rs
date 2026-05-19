//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1193/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1193<F: Float>(t13829: F, t193: F, t524: F, t1: F, t46873: F, t544: F, t1424: F, t42026: F, t42029: F, t42030: F, t42032: F, t48011: F, t48013: F, t48017: F, t48020: F, t48023: F, t48026: F) -> F {
    let t48029 = F::cast_from(0.35750489951850426669e0_f64) * t524 * t13829 * t193;
    let t48032 = t544 * t46873 * t1;
    let t48034 = F::cast_from(0.39722766613167140743e-1_f64) * t48032 * t1424;
    let t48037 = t48011 + t48013 + t48017 - t48020 + t48023 - t48026 + t48029 - F::cast_from(0.14896037479937677779e-1_f64) * t42026 - t48034 + t42029 + F::cast_from(0.35750489951850426669e0_f64) * t42030 + F::cast_from(0.35750489951850426669e0_f64) * t42032;
    t48037
}
