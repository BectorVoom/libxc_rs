//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1033/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1033<F: Float>(t1186: F, t19123: F, t19109: F, t3661: F, t2201: F, t3119: F, t2206: F, t3123: F, t16216: F, t5817: F, t158: F, t165: F, t173: F, t20711: F, t20714: F, t20718: F, t20719: F, t20721: F, t20724: F, t20727: F, t20730: F, t20733: F, t20736: F, t20739: F, t20740: F, t20743: F) -> (F,) {
    let t20746 = t1186 * t19123;
    let t20749 = t3661 * t19109;
    let t20752 = t3119 * t2201;
    let t20754 = t3123 * t2206;
    let t20756 = t16216 * t5817;
    let t20758 = -0.672175e-5 * t173 * t20711 + 0.22405833333333333333e-5 * t173 * t20714 - t20718 - 0.31226666666666666666e-2 * t20719 + 0.7026e-2 * t158 * t20721 - 0.7026e-2 * t158 * t20724 + 0.1171e-2 * t158 * t20727 + 0.4755e-2 * t165 * t20730 + 0.78066666666666666667e-3 * t158 * t20733 + 0.52833333333333333333e-2 * t20736 + t20739 - 0.1585e-2 * t165 * t20740 + 0.317e-2 * t165 * t20743 - 0.52833333333333333333e-3 * t165 * t20746 - 0.17611111111111111111e-3 * t165 * t20749 + 0.35222222222222222221e-2 * t20752 + 0.39210208333333333333e-4 * t20754 - 0.10038333333333333333e-1 * t20756;
    (t20758,)
}
