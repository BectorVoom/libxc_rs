//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1233/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1233<F: Float>(t111405: F, t111408: F, t111410: F, t111411: F, t111412: F, t111415: F, t116008: F, t116023: F, t1458: F, t1914: F, t1921: F, t2111: F, t2118: F, t25049: F, t25072: F, t3: F, t30627: F, t30663: F, t575: F, t6937: F, t6951: F, t8114: F, t8130: F) -> F {
    let tv4rho3sigma10 = t116008 * t3 * t575 + t116023 * t1458 + F::cast_from(3.0_f64) * t1914 * t30663 + F::cast_from(3.0_f64) * t1921 * t30627 + t2111 * t25072 + t2118 * t25049 + F::cast_from(3.0_f64) * t6937 * t8130 + F::cast_from(3.0_f64) * t6951 * t8114 + F::cast_from(3.0_f64) * t111405 + F::cast_from(6.0_f64) * t111408 + F::cast_from(3.0_f64) * t111410 + F::cast_from(3.0_f64) * t111411 + F::cast_from(6.0_f64) * t111412 + F::cast_from(3.0_f64) * t111415;
    tv4rho3sigma10
}
