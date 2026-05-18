//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1321/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1321<F: Float>(t2242: F, t4213: F, t53508: F, t53515: F, t2376: F, t26617: F, t4207: F, t810: F, t51153: F, t52309: F, t53487: F, t53493: F, t53498: F, t53510: F, t53513: F, t53517: F, t53520: F, t53526: F, t53529: F, t6793: F) -> F {
    let t55192 = t2242 * t4213;
    let t55195 = F::new(7.0) / F::new(36.0) * t53508;
    let t55198 = F::new(7.0) / F::new(36.0) * t53515;
    let t55204 = t26617 * t2376 * t4207 * t810;
    let t55208 = -F::new(7.0) / F::new(576.0) * t51153 - t53487 / F::new(8.0) - t53493 / F::new(384.0) + t53498 / F::new(384.0) - F::new(35.0) / F::new(432.0) * t55192 - F::new(7.0) / F::new(144.0) * t52309 + t55195 - t53510 / F::new(24.0) + t53513 / F::new(768.0) + t55198 + t53517 / F::new(12.0) - t53520 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t53526 - t6793 * t55204 / F::new(8.0) + t53529 / F::new(384.0);
    t55208
}
