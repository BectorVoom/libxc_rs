//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2852/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2852<F: Float>(t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t49930: F, t76963: F, t76966: F, t76967: F, t76969: F, t76970: F, t76973: F, t76974: F) -> F {
    let t76975 = t76963 + t39741 + t39744 + t39747 + t76966 + t39750 + t39756 + t39760 - t39764 + t39770 - t76967 + t49930 + t76969 + t76970 + t39773 - t76973 + t76974;
    t76975
}
