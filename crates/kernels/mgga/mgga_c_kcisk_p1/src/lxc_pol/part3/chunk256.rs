//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 256/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk256<F: Float>(t1161: F, t1184: F, t1165: F, t1176: F, t1181: F, t1188: F) -> F {
    let t1205 = F::cast_from(0.301925e0_f64) * t1161;
    let t1208 = F::cast_from(0.82785e-1_f64) * t1184;
    let t1210 = F::cast_from(0.258925e1_f64) * t1176 - t1205 - F::cast_from(0.301925e0_f64) * t1165 + F::cast_from(0.16504875e0_f64) * t1181 - t1208 - F::cast_from(0.82785e-1_f64) * t1188;
    t1210
}
