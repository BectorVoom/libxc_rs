//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1500/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500<F: Float>(t23177: F, t2798: F, t686: F, t72: F, t14568: F, t18730: F, t14586: F, t6016: F, t10529: F, t2782: F, t233: F, t23359: F, t689: F, t869: F) -> (F, F, F, F) {
    let t76100 = t2798 * t23177 * t72 * t686;
    let t76104 = t14568 * t18730;
    let t76106 = t14586 * t6016;
    let t76108 = t2782 * t10529 * t76106;
    let t76117 = t689 * t869 * t233 * t23359;
    (t76100, t76104, t76108, t76117)
}
