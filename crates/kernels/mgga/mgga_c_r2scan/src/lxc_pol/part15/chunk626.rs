//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 626/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk626<F: Float>(t1020: F, t1081: F, t333: F, t3648: F, t335: F, t337: F, t339: F, t341: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t343: F, t1035: F, t352: F) -> (F, F, F, F, F, F, F) {
    let t3650 = t1020 * t1081;
    let t3652 = t333 * t3648;
    let t3656 = t335 * t3648;
    let t3660 = t337 * t3648;
    let t3664 = t339 * t3648;
    let t3668 = t341 * t3648;
    let t3674 = -0.64e0 * t3648 - 0.8704e0 * t3650 - 0.8704e0 * t3652 - 0.9214113627294e1 * t1083 * t1020 - 0.4607056813647e1 * t3656 + 0.367387230261e2 * t1085 * t1020 + 0.122462410087e2 * t3660 - 0.3831420472412e2 * t1087 * t1020 - 0.957855118103e1 * t3664 + 0.1550653405116e2 * t1089 * t1020 + 0.3101306810232e1 * t3668 - 0.2177652951264e1 * t1091 * t1020 - 0.362942158544e0 * t343 * t3648;
    let t3675 = t352 * t1035;
    (t3652, t3656, t3660, t3664, t3668, t3674, t3675)
}
