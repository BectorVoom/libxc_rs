//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1471/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1471<F: Float>(t1357: F, t1904: F, t689: F, t1903: F, t72: F, t686: F) -> (F, F, F, F) {
    let t5718 = t1357 * t1904;
    let t5719 = t689 * t5718;
    let t5721 = t1903 * t72;
    let t5722 = t5721 * t686;
    (t5718, t5719, t5721, t5722)
}
