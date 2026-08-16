//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1357/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1357<F: Float>(t15327: F, t4414: F, t12213: F, t14622: F, t15207: F, t2409: F, t3066: F, t3207: F, t3721: F, t4016: F, t4052: F, t43526: F, t51819: F, t51825: F, t57311: F, t57319: F, t57324: F, t57326: F, t57330: F, t57332: F, t57334: F, t57338: F, t6781: F, t9296: F) -> F {
    let t57345 = t4414 * t15327;
    let t57347 = t3066 * t2409 * t12213 * t14622 / F::cast_from(24.0_f64) - t3207 * t2409 * t6781 * t15207 / F::cast_from(16.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t57311 + t3066 * t2409 * t43526 * t4016 / F::cast_from(48.0_f64) + t57319 / F::cast_from(3072.0_f64) + t57324 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57326 + t57330 / F::cast_from(768.0_f64) + t57332 / F::cast_from(24.0_f64) + t57334 / F::cast_from(8.0_f64) - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t51819 + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51825 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57338 - t3066 * t2409 * t9296 * t4052 * t3721 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57345;
    t57347
}
