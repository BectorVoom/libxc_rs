//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 490/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk490<F: Float>(t1275: F, t4100: F, t4101: F, t4007: F, t4060: F, t4011: F, t4015: F, t4018: F, t4021: F, t4039: F, t4047: F, t4055: F, t4057: F, t4063: F, t4067: F, t4070: F, t4073: F) -> (F, F) {
    let t4103 = t4100 * t4101 * t1275;
    let t4108 = 0.40256666666666666667e0 * t4007;
    let t4115 = 0.27595e0 * t4060;
    let t4120 = -0.1294625e1 * t4039 + 0.258925e1 * t4047 + t4108 + 0.20128333333333333334e0 * t4011 - 0.20128333333333333333e0 * t4015 + 0.60385e0 * t4018 - 0.301925e0 * t4021 + 0.82524375e-1 * t4055 + 0.16504875e0 * t4057 + t4115 + 0.22076e0 * t4063 - 0.5519e-1 * t4067 + 0.33114e0 * t4070 - 0.16557e0 * t4073;
    (t4103, t4120)
}
