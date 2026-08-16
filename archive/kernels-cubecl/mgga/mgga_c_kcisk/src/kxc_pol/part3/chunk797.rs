//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 797/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk797<F: Float>(t12313: F, t79: F, t781: F, t12230: F, t12237: F, t12242: F, t12248: F, t12249: F, t12251: F, t12258: F, t12263: F, t12266: F, t12269: F, t12273: F, t12277: F, t12281: F, t12287: F, t2005: F, t2013: F, t2025: F, t5465: F, t5511: F, t5517: F, t782: F, t788: F) -> F {
    let t12314 = t79 * t12313;
    let t12315 = t12314 * t781;
    let t12318 = F::cast_from(0.16191709844559585492e0_f64) * t2005 * t5511 - F::cast_from(0.8095854922279792746e-1_f64) * t5465 * t2025 - F::cast_from(0.53972366148531951639e-1_f64) * t12230 - F::cast_from(0.8095854922279792746e-1_f64) * t2005 * t5517 + F::cast_from(0.53972366148531951639e-1_f64) * t2013 * t12237 - F::cast_from(0.2698618307426597582e-1_f64) * t2013 * t12242 + t12248 + F::cast_from(0.2698618307426597582e-1_f64) * t12249 - F::cast_from(0.17990788716177317213e-1_f64) * t12251 - F::cast_from(0.16191709844559585492e0_f64) * t782 * t12258 + F::cast_from(0.17990788716177317213e-1_f64) * t12263 - F::cast_from(0.2698618307426597582e-1_f64) * t12266 + F::cast_from(0.53972366148531951639e-1_f64) * t12269 - F::cast_from(0.35981577432354634427e-1_f64) * t2013 * t12273 + F::cast_from(0.35981577432354634428e-1_f64) * t2013 * t12277 - F::cast_from(0.53972366148531951639e-1_f64) * t2013 * t12281 + F::cast_from(0.53972366148531951639e-1_f64) * t2013 * t12287 + F::cast_from(0.2698618307426597582e-1_f64) * t12315 * t788;
    t12318
}
