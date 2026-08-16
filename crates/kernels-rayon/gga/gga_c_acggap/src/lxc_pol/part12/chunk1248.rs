//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1248/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1248(t105: f64, t11179: f64, t14974: f64, t19409: f64, t2170: f64, t2408: f64, t32313: f64, t33333: f64, t33393: f64, t36577: f64, t36769: f64, t36771: f64, t36774: f64, t38013: f64, t38044: f64, t38072: f64, t38108: f64, t38149: f64, t38178: f64, t38204: f64, t38233: f64, t38270: f64, t38311: f64, t38336: f64, t38371: f64, t38406: f64, t38437: f64, t38469: f64, t38503: f64, t3984: f64, t469: f64, t567: f64, t7297: f64, t8031: f64, t8040: f64, t8048: f64, t8372: f64, t8382: f64) -> f64 {
    let t38514 = -6.0_f64 * t7297 * t11179 * t3984 - t567 * t2408 * t8031 - 6.0_f64 * t32313 + t33333 - t36769 + t36771 + t36774 - 3.0_f64 * t7297 * t8040 * t14974 + 3.0_f64 * t567 * t2170 * t33393 - 6.0_f64 * t8372 * t8040 * t36577 - 6.0_f64 * t7297 * t8040 * t19409 + t567 * t105 * (t38013 + t38044 + t38072 + t38108 + t38149 + t38178 + t38204 + t38233 + t38270 + t38311 + t38336 + t38371 + t38406 + t38437 + t38469 + t38503) * t469 + 6.0_f64 * t567 * t8048 * t8382;
    t38514
}
