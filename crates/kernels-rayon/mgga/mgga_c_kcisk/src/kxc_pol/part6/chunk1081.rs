//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1081/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1081(t31261: f64, t31263: f64, t31267: f64, t31269: f64, t31273: f64, t31275: f64, t31279: f64, t31281: f64, t31284: f64, t31288: f64, t31290: f64, t31293: f64, t31297: f64, t31301: f64, t31303: f64, t31396: f64, t31400: f64, t31402: f64) -> f64 {
    let t31795 = 0.1875e0_f64 * t31261 - 0.625e-1_f64 * t31263 - 0.5625e0_f64 * t31267 - 0.40468749999999999999e-1_f64 * t31269 + 0.60703125e-1_f64 * t31273 + 0.5625e0_f64 * t31275 - 0.13489583333333333333e-1_f64 * t31279 - 0.60703125e-1_f64 * t31281 + 0.625e-1_f64 * t31284 + 0.29976851851851851851e-2_f64 * t31288 - 0.13489583333333333333e-1_f64 * t31290 + 0.13489583333333333333e-1_f64 * t31293 - 0.9375e-1_f64 * t31297 + 0.625e-1_f64 * t31301 + 0.303515625e-1_f64 * t31303 + 0.9375e-1_f64 * t31396 + 0.101171875e-1_f64 * t31400 - 0.28125e0_f64 * t31402;
    t31795
}
