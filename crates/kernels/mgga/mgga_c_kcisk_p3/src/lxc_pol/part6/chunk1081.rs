//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1081/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1081<F: Float>(t31261: F, t31263: F, t31267: F, t31269: F, t31273: F, t31275: F, t31279: F, t31281: F, t31284: F, t31288: F, t31290: F, t31293: F, t31297: F, t31301: F, t31303: F, t31396: F, t31400: F, t31402: F) -> F {
    let t31795 = F::cast_from(0.1875e0_f64) * t31261 - F::cast_from(0.625e-1_f64) * t31263 - F::cast_from(0.5625e0_f64) * t31267 - F::cast_from(0.40468749999999999999e-1_f64) * t31269 + F::cast_from(0.60703125e-1_f64) * t31273 + F::cast_from(0.5625e0_f64) * t31275 - F::cast_from(0.13489583333333333333e-1_f64) * t31279 - F::cast_from(0.60703125e-1_f64) * t31281 + F::cast_from(0.625e-1_f64) * t31284 + F::cast_from(0.29976851851851851851e-2_f64) * t31288 - F::cast_from(0.13489583333333333333e-1_f64) * t31290 + F::cast_from(0.13489583333333333333e-1_f64) * t31293 - F::cast_from(0.9375e-1_f64) * t31297 + F::cast_from(0.625e-1_f64) * t31301 + F::cast_from(0.303515625e-1_f64) * t31303 + F::cast_from(0.9375e-1_f64) * t31396 + F::cast_from(0.101171875e-1_f64) * t31400 - F::cast_from(0.28125e0_f64) * t31402;
    t31795
}
