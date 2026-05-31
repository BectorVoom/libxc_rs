//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 538/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk538<F: Float>(t1288: F, t2901: F, t2905: F, t2917: F, t2921: F, t313: F) -> F {
    let t2938 = F::cast_from(3.0_f64) / F::cast_from(10.0_f64) * t313 * (F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t2901 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t2905 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t2917 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t2921) + t1288;
    t2938
}
