//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3073/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073<F: Float>(t2923: F, t4587: F, t11384: F, t1596: F, t11466: F, t300: F, t11452: F, t4669: F, t11450: F, t1621: F, t11507: F, t1633: F) -> (F, F, F, F, F, F) {
    let t52219 = t4587 * t2923;
    let t52224 = t1596 * t11384;
    let t52238 = t300 * t11466;
    let t52264 = t4669 * t11452;
    let t52320 = t11450 * t1621;
    let t52370 = t11507 * t1633;
    (t52219, t52224, t52238, t52264, t52320, t52370)
}
