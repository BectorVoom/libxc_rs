//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 927/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk927<F: Float>(t1317: F, t34397: F, t376: F, t136151: F, t144893: F, t32067: F, t144857: F, t2258: F, t7243: F, t136138: F, t144813: F, t34495: F, t89: F, t34491: F, t34503: F, t22873: F, t28: F, t6454: F) -> (F, F, F, F, F, F, F, F) {
    let t145667 = t1317 * t376 * t34397;
    let t145669 = t32067 * t136151 * t144893;
    let t145673 = t32067 * t2258 * t7243 * t144857;
    let t145676 = t32067 * t136138 * t144813;
    let t145681 = t89 * t376 * t34495;
    let t145684 = t89 * t376 * t34491;
    let t145687 = t89 * t376 * t34503;
    let t145691 = t89 * t28 * t22873 * t6454;
    (t145667, t145669, t145673, t145676, t145681, t145684, t145687, t145691)
}
