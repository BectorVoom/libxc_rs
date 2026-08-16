//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3171/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3171<F: Float>(t12808: F, t21013: F, t1222: F, t3698: F, t5047: F, t697: F, t12855: F, t12916: F, t17455: F, t16738: F, t17240: F, t16742: F) -> (F, F, F, F, F) {
    let t57710 = t12808 * t21013;
    let t57726 = t1222 * t697 * t3698 * t5047;
    let t57735 = t12855 * t12916 * t17455;
    let t57743 = t1222 * t17240 * t16738;
    let t57746 = t1222 * t17240 * t16742;
    (t57710, t57726, t57735, t57743, t57746)
}
