//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1626/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1626<F: Float>(t12901: F, t13033: F, t13042: F, t13047: F, t3172: F, t3555: F, t3781: F, t5330: F, t12861: F, t12916: F, t3718: F, t11262: F, t3600: F, t3605: F) -> (F, F, F, F, F) {
    let t44658 = t13033 * t12901;
    let t44661 = t13042 * t3172 * t13047;
    let t44664 = t3555 * t3781 * t5330;
    let t44672 = t3718 * t12916 * t12861;
    let t44675 = t3600 * t11262 * t3605;
    (t44658, t44661, t44664, t44672, t44675)
}
