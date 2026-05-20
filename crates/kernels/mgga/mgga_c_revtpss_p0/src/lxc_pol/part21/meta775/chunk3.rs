//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2761/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2761<F: Float>(t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t49898: F, t49912: F, t49913: F) -> F {
    let t50845 = -t49898 - t39483 + t39520 + t49912 - t39528 + t39531 + t39534 + t39537 - t39540 + t49913 + t39741 + t39744 + t39747 + t39750 + t39756 + t39760 - t39764;
    t50845
}
