//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1940/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1940<F: Float>(t13509: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t13448: F, t13451: F, t13453: F, t13455: F, t13459: F, t13462: F, t69: F) -> (F, F) {
    let t13510 = t655 * t13509;
    let t13513 = -t10201 - F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t10202 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10204 + t10206 / F::cast_from(3.0_f64) - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t13448 - t13451 + t13453 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t69 * t13455 + t69 * t13459 / F::cast_from(2.0_f64) + t69 * t13462 / F::cast_from(4.0_f64) - t69 * t13510 / F::cast_from(8.0_f64);
    (t13510, t13513)
}
