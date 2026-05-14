//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1134/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1134<F: Float>(t3222: F, t3721: F, t3972: F, t51548: F, t3733: F, t54580: F, t13781: F, t3808: F, t52000: F, t13782: F, t3861: F, t2306: F, t3037: F, t3975: F, t9385: F, t15144: F, t3038: F) -> (F, F, F, F, F, F) {
    let t56618 = t3972 * t51548 * t3721 * param_a_c * t3222;
    let t56626 = t54580 * t3733;
    let t56638 = t3972 * t13781 * t3808 * t52000;
    let t56642 = t3972 * t13781 * t3861 * t13782;
    let t56647 = t3972 * t3975 * t9385 * t2306 * t3037;
    let t56651 = t3972 * t13781 * t3038 * t15144;
    (t56618, t56626, t56638, t56642, t56647, t56651)
}
