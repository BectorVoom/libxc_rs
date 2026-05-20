//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2880/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880<F: Float>(t1610: F, t41571: F, t11289: F, t4632: F, t11510: F, t1633: F, t41224: F, t981: F, t15573: F, t3022: F, t11466: F, t300: F) -> (F, F, F, F, F) {
    let t52229 = F::new(1.0) * t41571 * t1610;
    let t52231 = F::new(3.0) * t11289 * t4632;
    let t52235 = F::cast_from(0.12304822629859687989e5_f64) * t981 * t41224 * t1633 * t11510;
    let t52237 = F::cast_from(0.31168546390226634765e3_f64) * t3022 * t15573;
    let t52238 = t300 * t11466;
    (t52229, t52231, t52235, t52237, t52238)
}
