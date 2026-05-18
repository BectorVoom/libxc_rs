//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1364/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1364<F: Float>(t1144: F, t15034: F, t859: F, t12237: F, t14185: F, t14952: F, t15526: F, t2408: F, t2409: F, t3066: F, t3189: F, t3207: F, t4228: F, t53544: F, t55212: F, t55218: F, t55228: F, t56548: F, t56551: F, t56553: F, t56555: F, t56560: F, t56578: F, t6793: F, t8589: F, t8734: F, t9283: F) -> F {
    let t58201 = t859 * t1144 * t15034;
    let t58224 = -t56548 / F::new(384.0) + t56551 / F::new(96.0) + t6793 * t58201 / F::new(24.0) + t55212 - F::new(35.0) / F::new(576.0) * t56553 + t55218 + t56555 / F::new(24.0) - t3207 * t9283 * t4228 * t3189 / F::new(8.0) + t55228 + t3207 * t9283 * t14185 * t12237 / F::new(8.0) - F::new(7.0) / F::new(1152.0) * t56560 - t53544 + t2408 * t2409 * t8589 * t14952 / F::new(24.0) + t3066 * t2409 * t8734 * t15526 / F::new(48.0) + t56578 / F::new(48.0);
    t58224
}
