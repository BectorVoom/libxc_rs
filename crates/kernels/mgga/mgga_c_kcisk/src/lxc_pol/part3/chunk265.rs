//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 265/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk265<F: Float>(t1163: F, t1248: F, t1249: F, t1227: F, t1238: F, t1240: F, t1243: F, t1247: F, t360: F) -> (F, F, F) {
    let t1251 = t1248 * t1249 * t1163;
    let t1253 = F::new(0.1898925e1) * t1238 - t1240 - F::cast_from(0.29896666666666666667e0_f64) * t1227 + F::new(0.3071625e0) * t1243 - t1247 - F::cast_from(0.16431333333333333333e0_f64) * t1251;
    let t1254 = F::new(1.0) / t360;
    (t1251, t1253, t1254)
}
