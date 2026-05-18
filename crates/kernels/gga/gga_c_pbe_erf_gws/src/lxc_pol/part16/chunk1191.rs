//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1191/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1191<F: Float>(t14091: F, t51329: F, t14028: F, t2273: F, t2209: F, t4021: F, t6562: F, t2196: F, t3065: F, t14046: F, t2173: F, t3969: F, t916: F) -> (F, F, F, F, F, F) {
    let t51330 = t14091 * t51329;
    let t51332 = t14028 * t2273;
    let t51334 = t4021 * t2209;
    let t51335 = t51334 * t6562;
    let t51338 = t3065 * t2196;
    let t51341 = t14046 * t2173;
    let t51350 = t3969 * t916;
    (t51330, t51332, t51335, t51338, t51341, t51350)
}
