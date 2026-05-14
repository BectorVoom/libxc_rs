//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1049/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1049<F: Float>(t14374: F, t79: F, t19881: F, t6321: F, t14592: F, t6373: F, t14591: F, t491: F, t6323: F, t20957: F, t4231: F, t6368: F, t2278: F, t4181: F, t493: F, t19895: F, t6322: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21029 = t79 * t14374;
    let t21030 = t21029 * t19881;
    let t21031 = t6321 * t21030;
    let t21033 = t14592 * t6373;
    let t21035 = t491 * t14591;
    let t21036 = t21035 * t6323;
    let t21038 = t4231 * t20957;
    let t21039 = t6368 * t21038;
    let t21041 = t4181 * t2278;
    let t21042 = t493 * t21041;
    let t21044 = t6322 * t19895;
    (t21030, t21031, t21033, t21036, t21038, t21039, t21041, t21042, t21044)
}
