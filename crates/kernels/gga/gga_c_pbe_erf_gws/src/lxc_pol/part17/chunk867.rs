//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 867/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk867<F: Float>(t7341: F, t7435: F, t587: F, t1407: F, t2565: F, t1827: F, t1017: F, t1663: F, t1403: F, t5543: F, t1416: F, t2570: F) -> (F, F, F, F) {
    let t7436 = t7435 * t7341;
    let t7438 = F::new(32.0) / F::new(81.0) * t587 * t7436;
    let t7439 = t2565 * t1407;
    let t7440 = t1827 * t7439;
    let t7442 = F::new(4.0) / F::new(45.0) * t587 * t7440;
    let t7443 = t1017 * t1663;
    let t7444 = t7443 * t1403;
    let t7445 = t5543 * t7444;
    let t7447 = F::new(4.0) / F::new(27.0) * t587 * t7445;
    let t7448 = t2570 * t1416;
    (t7438, t7442, t7447, t7448)
}
