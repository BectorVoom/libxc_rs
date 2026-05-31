//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2697/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697<F: Float>(t1868: F, t4135: F, t13586: F, t3889: F, t39799: F, t4139: F, t47059: F, t48265: F, t48266: F, t48268: F, t48270: F, t48271: F, t48275: F, t5536: F, t5537: F, t7315: F, t9628: F) -> F {
    let t49582 = t1868 * t4135;
    let t49592 = F::cast_from(18.0_f64) * t13586 * t3889 * t5536 - F::cast_from(9.0_f64) * t4139 * t49582 * t7315 + F::cast_from(6.0_f64) * t5536 * t5537 * t9628 + t39799 + t47059 - t48265 - t48266 + t48268 - t48270 - t48271 + t48275;
    t49592
}
