//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 723/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk723<F: Float>(t1203: F, t2142: F, t7637: F, t2147: F, t3565: F, t7635: F) -> (F, F, F) {
    let t7638 = t2142 * t1203;
    let t7639 = t7637 * t7638;
    let t7642 = t2147 * t3565;
    let t7643 = t7642 * t7635;
    (t7639, t7642, t7643)
}
