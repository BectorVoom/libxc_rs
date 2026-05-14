//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1050/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1050<F: Float>(t31846: F, t4426: F, t119777: F, t4430: F, t119788: F, t1558: F, t867: F, t119781: F, t119783: F, t247: F, t126046: F, t837: F, t33711: F, t846: F, t1568: F, t31805: F) -> (F, F, F, F, F, F, F) {
    let t126085 = t31846 * t4426;
    let t126087 = t119777 * t4430;
    let t126089 = t119788 * t4430;
    let t126092 = t867 * t1558;
    let t126095 = t119781 * t247 * t126092 * t119783;
    let t126099 = t119781 * t247 * t126046 * t837;
    let t126108 = t33711 * t846;
    let t126110 = t31805 * t1568;
    (t126085, t126087, t126089, t126095, t126099, t126108, t126110)
}
