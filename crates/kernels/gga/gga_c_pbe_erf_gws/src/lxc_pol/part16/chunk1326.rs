//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1326/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1326<F: Float>(t53725: F, t53727: F, t1206: F, t15082: F, t3189: F, t3207: F, t335: F, t338: F, t4111: F, t51564: F, t53677: F, t53689: F, t53691: F, t53693: F, t53695: F, t53700: F, t53713: F, t53715: F, t53721: F, t8804: F, t892: F, t9283: F) -> F {
    let t55344 = F::new(7.0) / F::new(72.0) * t53725;
    let t55345 = F::new(7.0) / F::new(1152.0) * t53727;
    let t55350 = -t53677 / F::new(24.0) + F::new(7.0) / F::new(576.0) * t51564 - t53689 / F::new(24.0) - t53691 / F::new(48.0) - t53693 / F::new(12.0) + t53695 / F::new(24.0) - t53700 / F::new(48.0) - t53713 / F::new(256.0) + t53715 / F::new(48.0) - t53721 / F::new(768.0) - t3207 * t9283 * t4111 * t3189 / F::new(8.0) - t3207 * t9283 * t1206 * t8804 / F::new(8.0) - t55344 + t55345 - t335 * t338 * t892 * t15082 / F::new(48.0);
    t55350
}
