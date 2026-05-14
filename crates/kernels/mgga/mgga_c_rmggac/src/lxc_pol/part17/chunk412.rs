//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 412/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk412<F: Float>(t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F, t402: F, t4052: F, t417: F, t171: F, t4058: F, t1041: F, t4151: F, t418: F, t971: F) -> (F, F, F, F, F, F) {
    let t4305 = -0.47063e1 * t4130 + 0.31375333333333333334e1 * t4133 - 0.36604555555555555556e1 * t4136 - 0.16068111111111111111e1 * t4138 + 0.28051666666666666666e0 * t4142 - 0.56103333333333333332e0 * t4144 - 0.6545388888888888889e0 * t4146 - 0.46308888888888888888e0 * t4149;
    let t4306 = t4305 * t402;
    let t4309 = t4052 * t417;
    let t4312 = t171 * t4058;
    let t4313 = t4052 * t1041;
    let t4316 = t4151 * t417;
    let t4319 = t418 * t971;
    (t4306, t4309, t4312, t4313, t4316, t4319)
}
