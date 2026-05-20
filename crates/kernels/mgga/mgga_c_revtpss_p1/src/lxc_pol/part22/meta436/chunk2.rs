//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2066/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2066<F: Float>(t1544: F, t836: F, t2749: F, t14785: F, t2746: F, t828: F) -> (F, F, F, F) {
    let t14786 = t1544 * t836;
    let t14787 = t14786 * t2749;
    let t14788 = t14785 * t14787;
    let t14791 = t2746 * t828;
    (t14786, t14787, t14788, t14791)
}
