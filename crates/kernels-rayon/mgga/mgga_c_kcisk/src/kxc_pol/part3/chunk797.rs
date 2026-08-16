//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 797/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk797(t12313: f64, t79: f64, t781: f64, t12230: f64, t12237: f64, t12242: f64, t12248: f64, t12249: f64, t12251: f64, t12258: f64, t12263: f64, t12266: f64, t12269: f64, t12273: f64, t12277: f64, t12281: f64, t12287: f64, t2005: f64, t2013: f64, t2025: f64, t5465: f64, t5511: f64, t5517: f64, t782: f64, t788: f64) -> f64 {
    let t12314 = t79 * t12313;
    let t12315 = t12314 * t781;
    let t12318 = 0.16191709844559585492e0_f64 * t2005 * t5511 - 0.8095854922279792746e-1_f64 * t5465 * t2025 - 0.53972366148531951639e-1_f64 * t12230 - 0.8095854922279792746e-1_f64 * t2005 * t5517 + 0.53972366148531951639e-1_f64 * t2013 * t12237 - 0.2698618307426597582e-1_f64 * t2013 * t12242 + t12248 + 0.2698618307426597582e-1_f64 * t12249 - 0.17990788716177317213e-1_f64 * t12251 - 0.16191709844559585492e0_f64 * t782 * t12258 + 0.17990788716177317213e-1_f64 * t12263 - 0.2698618307426597582e-1_f64 * t12266 + 0.53972366148531951639e-1_f64 * t12269 - 0.35981577432354634427e-1_f64 * t2013 * t12273 + 0.35981577432354634428e-1_f64 * t2013 * t12277 - 0.53972366148531951639e-1_f64 * t2013 * t12281 + 0.53972366148531951639e-1_f64 * t2013 * t12287 + 0.2698618307426597582e-1_f64 * t12315 * t788;
    t12318
}
