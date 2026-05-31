//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1248/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1248<F: Float>(t105: F, t11179: F, t14974: F, t19409: F, t2170: F, t2408: F, t32313: F, t33333: F, t33393: F, t36577: F, t36769: F, t36771: F, t36774: F, t38013: F, t38044: F, t38072: F, t38108: F, t38149: F, t38178: F, t38204: F, t38233: F, t38270: F, t38311: F, t38336: F, t38371: F, t38406: F, t38437: F, t38469: F, t38503: F, t3984: F, t469: F, t567: F, t7297: F, t8031: F, t8040: F, t8048: F, t8372: F, t8382: F) -> F {
    let t38514 = -F::cast_from(6.0_f64) * t7297 * t11179 * t3984 - t567 * t2408 * t8031 - F::cast_from(6.0_f64) * t32313 + t33333 - t36769 + t36771 + t36774 - F::cast_from(3.0_f64) * t7297 * t8040 * t14974 + F::cast_from(3.0_f64) * t567 * t2170 * t33393 - F::cast_from(6.0_f64) * t8372 * t8040 * t36577 - F::cast_from(6.0_f64) * t7297 * t8040 * t19409 + t567 * t105 * (t38013 + t38044 + t38072 + t38108 + t38149 + t38178 + t38204 + t38233 + t38270 + t38311 + t38336 + t38371 + t38406 + t38437 + t38469 + t38503) * t469 + F::cast_from(6.0_f64) * t567 * t8048 * t8382;
    t38514
}
