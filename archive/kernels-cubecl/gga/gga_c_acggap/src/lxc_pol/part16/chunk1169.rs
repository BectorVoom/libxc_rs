//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1169/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1169<F: Float>(t13299: F, t31115: F, t40116: F, t1788: F, t31110: F, t2041: F, t5632: F, t31495: F, t31499: F, t31505: F, t31509: F, t35673: F, t35679: F, t35683: F, t35686: F, t35703: F, t35710: F, t40105: F, t40107: F, t40109: F, t40111: F, t40114: F) -> F {
    let t40118 = t31115 * t13299 * t40116;
    let t40121 = t31110 * t1788;
    let t40123 = t2041 * t5632;
    let t40125 = -t35673 + F::cast_from(0.17149607247227894789e-1_f64) * t40105 + t35679 - F::cast_from(0.17149607247227894789e-1_f64) * t40107 - F::cast_from(0.17149607247227894789e-2_f64) * t40109 - F::cast_from(0.17149607247227894789e-2_f64) * t40111 - t35683 - t35686 + F::cast_from(0.42874018118069736972e-3_f64) * t40114 + t35703 + F::cast_from(0.15724046144802076034e-2_f64) * t40118 - t31495 - t31499 - t35710 - F::cast_from(0.90035438047946447642e-2_f64) * t31505 - t31509 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t40121 - t40123 / F::cast_from(48.0_f64);
    t40125
}
