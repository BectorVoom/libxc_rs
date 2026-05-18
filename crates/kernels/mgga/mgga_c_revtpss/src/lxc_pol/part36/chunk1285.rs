//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1285/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1285<F: Float>(t22068: F, t25972: F, t25978: F, t6880: F, t6856: F, t1904: F, t27985: F, t689: F, t1927: F, t5816: F, t13272: F, t1470: F) -> (F, F, F, F, F, F) {
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    let t108629 = t25978 * t6856;
    let t108662 = t689 * t27985 * t1904;
    let t108879 = t1927 * t5816;
    let t108966 = t13272 * t1470;
    (t108625, t108627, t108629, t108662, t108879, t108966)
}
