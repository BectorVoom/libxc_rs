//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 582/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk582<F: Float>(t10: F, t1337: F, t1224: F, t3575: F, t1225: F, t3579: F, t3583: F, t4008: F, t4011: F, t1229: F, t1233: F, t1232: F, t357: F, t346: F, t1253: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4013 = t10 * t1337;
    let t4015 = t1224 * t4013 * t3575;
    let t4018 = t1224 * t1225 * t3579;
    let t4021 = t1224 * t1225 * t3583;
    let t4023 = t4008 + 0.11872222222222222222e-1 * t4011 - 0.11872222222222222222e-1 * t4015 + 0.35616666666666666666e-1 * t4018 - 0.17808333333333333333e-1 * t4021;
    let t4026 = t1229 * t1233;
    let t4029 = t1232 * t357;
    let t4030 = 1.0 / t4029;
    let t4031 = t346 * t4030;
    let t4032 = t1253 * t1253;
    (t4013, t4015, t4018, t4021, t4023, t4026, t4030, t4031, t4032)
}
