//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 718/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk718<F: Float>(t198: F, t3663: F, t3662: F, t1123: F, t1555: F, t1161: F, t1129: F, t1159: F, t1535: F, t2824: F, t1143: F, t525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3664 = t3663 * t198;
    let t3665 = t3662 * t3664;
    let t3668 = t1555 * t1123;
    let t3669 = t1161 * t3668;
    let t3672 = t1555 * t1129;
    let t3673 = t1161 * t3672;
    let t3676 = t1159 * t1535;
    let t3677 = t3676 * t2824;
    let t3680 = t1143 * t525;
    (t3664, t3665, t3668, t3669, t3672, t3673, t3676, t3677, t3680)
}
