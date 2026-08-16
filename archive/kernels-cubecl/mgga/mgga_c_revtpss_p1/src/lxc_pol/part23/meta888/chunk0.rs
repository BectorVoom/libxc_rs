//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2812/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812<F: Float>(t10529: F, t2782: F, t76106: F, t233: F, t23359: F, t689: F, t869: F, t14598: F, t23160: F, t686: F, t72: F, t23244: F, t251: F) -> (F, F, F, F) {
    let t76108 = t2782 * t10529 * t76106;
    let t76117 = t689 * t869 * t233 * t23359;
    let t76125 = t14598 * t23160 * t72 * t686;
    let t76127 = t251 * t23244;
    (t76108, t76117, t76125, t76127)
}
