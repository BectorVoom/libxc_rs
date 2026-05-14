//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1144/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1144<F: Float>(t13781: F, t3808: F, t3972: F, t52000: F, t13782: F, t3861: F, t2306: F, t3037: F, t3975: F, t9385: F, t15144: F, t3038: F, t1115: F, t12255: F, t13939: F, t14437: F, t14791: F, t2408: F, t2498: F, t3040: F, t3913: F, t4002: F, t52897: F, t53681: F, t56604: F, t56613: F, t56618: F, t56620: F, t56626: F, t9283: F, t9958: F) -> (F,) {
    let t56638 = t3972 * t13781 * t3808 * t52000;
    let t56642 = t3972 * t13781 * t3861 * t13782;
    let t56647 = t3972 * t3975 * t9385 * t2306 * t3037;
    let t56651 = t3972 * t13781 * t3038 * t15144;
    let t56653 = t56604 / 384.0 + t2408 * t9283 * t14791 * t12255 / 8.0 - t56613 / 1536.0 + t56618 / 768.0 + 7.0 / 144.0 * t56620 - t3913 * t13939 / 96.0 - t1115 * t52897 / 48.0 - t56626 / 96.0 - t9958 * t4002 / 96.0 - t3040 * t14437 / 48.0 - t2498 * t14437 / 48.0 - t1115 * t53681 / 48.0 - t56638 / 768.0 - t56642 / 1536.0 + t56647 / 384.0 - t56651 / 768.0;
    (t56653,)
}
