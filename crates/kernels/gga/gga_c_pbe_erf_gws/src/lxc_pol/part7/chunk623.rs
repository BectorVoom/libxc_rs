//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 623/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk623<F: Float>(t5038: F, t657: F, t1699: F, t395: F, t191: F, t784: F, t190: F, t212: F, t4373: F, t626: F, t1251: F, t658: F, t1721: F, t401: F, t1715: F, t25: F, t5022: F, t5025: F, t5030: F, t5034: F) -> (F, F, F, F, F, F) {
    let t5039 = t657 * t5038;
    let t5042 = t395 * t1699;
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1 * t190 * t5044 * t212;
    let t5048 = t626 * t4373;
    let t5049 = t657 * t5048;
    let t5052 = t1251 * t658;
    let t5054 = t401 * t1721;
    let t5056 = t401 * t1715;
    let t5058 = -0.26666666666666666667e-1 * t5022 + 0.13333333333333333333e-1 * t25 * t5025 - 0.66666666666666666666e-2 * t25 * t5030 - 0.39999999999999999999e-1 * t25 * t5034 + 0.39999999999999999999e-1 * t25 * t5039 - 0.71983333333333333333e-1 * t5042 - t5047 - 0.66666666666666666667e-2 * t25 * t5049 - 0.22222222222222222222e-1 * t5052 + 0.13333333333333333334e-1 * t5054 + 0.44444444444444444445e-2 * t5056;
    (t5039, t5042, t5044, t5048, t5049, t5058)
}
