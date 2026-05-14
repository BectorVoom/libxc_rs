//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1010/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1010<F: Float>(t2053: F, t4058: F, t1198: F, t6854: F, t1105: F, t13751: F, t944: F, t2494: F, t3944: F, t4188: F, t945: F, t810: F, t1192: F, t8589: F, t829: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t14149 = t4058 * t2053;
    let t14153 = t1198 * t6854;
    let t14380 = t13751 * t1105;
    let t14383 = t1105 * t944;
    let t14387 = t3944 * t2494;
    let t14390 = t4188 * t945;
    let t14392 = t14390 * t810;
    let t14395 = t8589 * t1192;
    let t14397 = t829 * t830 * t14395;
    (t14149, t14153, t14380, t14383, t14387, t14390, t14392, t14397)
}
