//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 638/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk638<F: Float>(t25: F, t4960: F, t4965: F, t5248: F, t5250: F, t5253: F, t5256: F, t5258: F, t5260: F, t5265: F, t5268: F, t5271: F, t5245: F, t598: F, t186: F, t185: F) -> (F, F, F, F) {
    let t5272 = 0.14396666666666666667e0 * t4960 - 0.71983333333333333335e-1 * t4965 - 0.26666666666666666667e-1 * t5248 + 0.13333333333333333333e-1 * t25 * t5250 - 0.66666666666666666666e-2 * t25 * t5253 - 0.22222222222222222222e-1 * t5256 + 0.13333333333333333334e-1 * t5258 + 0.44444444444444444445e-2 * t5260 - 0.29629629629629629629e-2 * t25 * t5265 - 0.66666666666666666667e-2 * t25 * t5268 - t5271;
    let t5273 = t5245 + t5272;
    let t5274 = t598 * t5273;
    let t5275 = t186 * t5274;
    let t5277 = 2.0 / 15.0 * t185 * t5275;
    (t5273, t5274, t5275, t5277)
}
