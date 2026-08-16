//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2458/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458(t10956: f64, t1611: f64, t10517: f64, t4630: f64, t10459: f64, t4644: f64, t4608: f64, t698: f64, t973: f64, t10398: f64, t1041: f64, t10419: f64, t1044: f64, t1046: f64, t13995: f64, t14085: f64, t14147: f64, t14187: f64, t14189: f64, t248: f64, t3057: f64, t3117: f64, t43301: f64, t4582: f64, t4588: f64, t45997: f64, t47734: f64, t48554: f64) -> f64 {
    let t50334 = t1611 * t10956;
    let t50337 = t10517 * t4630;
    let t50343 = t4644 * t10459;
    let t50361 = t973 * t698 * t4608;
    let t50362 = t50361 / 432.0_f64;
    let t50365 = t13995 * t10398 / 1536.0_f64 + 19.0_f64 / 864.0_f64 * t50334 * t1046 + 19.0_f64 / 864.0_f64 * t50337 + t1041 * t248 * t1044 * t47734 / 4608.0_f64 + 5.0_f64 / 6912.0_f64 * t50343 + t14085 * t3057 / 1536.0_f64 - t3117 * t14147 / 768.0_f64 + 5.0_f64 / 1728.0_f64 * t3117 * t14189 + 5.0_f64 / 4608.0_f64 * t1041 * t4582 * t4588 * t45997 + 5.0_f64 / 1728.0_f64 * t1041 * t4582 * t14187 * t48554 + t43301 / 1536.0_f64 - t50362 - t13995 * t10419 / 768.0_f64;
    t50365
}
