//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1964/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1964<F: Float>(t1398: F, t1868: F, t3938: F, t13783: F, t3935: F, t828: F) -> (F, F, F, F) {
    let t13784 = t1868 * t1398;
    let t13785 = t13784 * t3938;
    let t13786 = t13783 * t13785;
    let t13789 = t3935 * t828;
    (t13784, t13785, t13786, t13789)
}
