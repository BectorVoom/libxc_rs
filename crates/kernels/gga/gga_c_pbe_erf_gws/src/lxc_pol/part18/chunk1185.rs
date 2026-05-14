//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1185/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1185<F: Float>(t14121: F, t9948: F, t15282: F, t51666: F, t15327: F, t4414: F, t12213: F, t14622: F, t15207: F, t2409: F, t3066: F, t3207: F, t3721: F, t4016: F, t4052: F, t43526: F, t51819: F, t51825: F, t57311: F, t57319: F, t57324: F, t57326: F, t57330: F, t57332: F, t6781: F, t9296: F) -> (F,) {
    let t57334 = t14121 * t9948;
    let t57338 = t51666 * t15282;
    let t57345 = t4414 * t15327;
    let t57347 = t3066 * t2409 * t12213 * t14622 / 24.0 - t3207 * t2409 * t6781 * t15207 / 16.0 + 5.0 / 768.0 * t57311 + t3066 * t2409 * t43526 * t4016 / 48.0 + t57319 / 3072.0 + t57324 / 768.0 - 7.0 / 144.0 * t57326 + t57330 / 768.0 + t57332 / 24.0 + t57334 / 8.0 - 119.0 / 13824.0 * t51819 + 35.0 / 216.0 * t51825 - 7.0 / 576.0 * t57338 - t3066 * t2409 * t9296 * t4052 * t3721 / 16.0 - 7.0 / 144.0 * t57345;
    (t57347,)
}
