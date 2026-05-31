//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2867/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2867<F: Float>(t52035: F, t52037: F, t2852: F, t373: F, t51957: F, t51959: F) -> (F, F, F) {
    let t52091 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t52035;
    let t52092 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t52037;
    let t52110 = t373 * t2852;
    let t52112 = t51957 * t52110 * t51959;
    (t52091, t52092, t52112)
}
