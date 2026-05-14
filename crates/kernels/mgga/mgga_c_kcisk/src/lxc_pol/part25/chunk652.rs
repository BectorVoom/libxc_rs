//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 652/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk652<F: Float>(t1842: F, t6764: F, t1659: F, t6759: F, t158: F, t165: F, t220: F, t5134: F, t5135: F, t5816: F, t5823: F, t5827: F, t6906: F, t6910: F, t6913: F, t6916: F, t6919: F, t6922: F, t6924: F, t6926: F, t6928: F, t6931: F) -> (F, F, F) {
    let t6934 = t1842 * t6764;
    let t6937 = t1659 * t6759;
    let t6940 = 0.10359077815592613752e-3 * t6906 - 0.23911438650126355246e-1 * t5134 * t220 + 0.10359077815592613752e-3 * t6910 * t220 + 0.7026e-2 * t5827 * t6913 - 0.1585e-2 * t5816 * t6916 - 0.10082625e-4 * t5823 * t6919 + 0.4684e-2 * t6922 - 0.13208333333333333333e-2 * t6924 - 0.117630625e-4 * t6926 + 0.7026e-2 * t158 * t6928 + 0.1171e-2 * t158 * t6931 - 0.1585e-2 * t165 * t6934 - 0.52833333333333333333e-3 * t165 * t6937 + t5135;
    (t6934, t6937, t6940)
}
