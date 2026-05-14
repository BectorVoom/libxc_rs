//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 799/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk799<F: Float>(t1027: F, t1793: F, t4927: F, t639: F, t2559: F, t7336: F, t587: F, t197: F, t5293: F, t7341: F, t1407: F, t2565: F, t1827: F, t1017: F, t1663: F, t1403: F) -> (F, F, F, F, F) {
    let t7428 = t1027 * t1793;
    let t7429 = t4927 * t7428;
    let t7431 = 8.0 / 45.0 * t639 * t7429;
    let t7432 = t2559 * t7336;
    let t7434 = 4.0 / 27.0 * t587 * t7432;
    let t7435 = t5293 * t197;
    let t7436 = t7435 * t7341;
    let t7438 = 32.0 / 81.0 * t587 * t7436;
    let t7439 = t2565 * t1407;
    let t7440 = t1827 * t7439;
    let t7442 = 4.0 / 45.0 * t587 * t7440;
    let t7443 = t1017 * t1663;
    let t7444 = t7443 * t1403;
    (t7431, t7434, t7438, t7442, t7444)
}
