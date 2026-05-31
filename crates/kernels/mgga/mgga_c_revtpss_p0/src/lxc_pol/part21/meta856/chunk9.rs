//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3256/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3256<F: Float>(t10259: F, t10416: F, t10426: F, t1310: F, t13207: F, t13216: F, t13425: F, t13426: F, t13517: F, t13537: F, t13544: F, t1453: F, t1518: F, t18227: F, t1843: F, t1911: F, t2322: F, t2372: F, t4248: F, t4254: F, t4297: F, t508: F, t569: F, t60499: F, t60556: F, t651: F) -> F {
    let t60558 = -F::cast_from(2.0_f64) * t10259 * t1843 * t651 - F::cast_from(2.0_f64) * t13207 * t1518 * t651 - F::cast_from(6.0_f64) * t10416 * t4297 + t10426 * t1911 - F::cast_from(3.0_f64) * t1310 * t13425 - F::cast_from(6.0_f64) * t13216 * t4248 - F::cast_from(6.0_f64) * t13426 * t2372 + F::cast_from(3.0_f64) * t13517 * t1453 - F::cast_from(6.0_f64) * t13537 * t2322 - F::cast_from(6.0_f64) * t13537 * t4254 - F::cast_from(6.0_f64) * t13544 * t4254 - F::cast_from(6.0_f64) * t18227 * t2372 - t508 * t60499 + t569 * t60556;
    t60558
}
