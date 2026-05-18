//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1321/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1321<F: Float>(t1184: F, t8975: F, t51351: F, t9509: F, t51383: F, t51401: F, t54293: F, t54294: F, t54295: F, t54297: F, t54299: F, t54302: F, t54303: F, t54305: F) -> F {
    let t54307 = t1184 * t8975;
    let t54310 = t51351 * t9509;
    let t54312 = -F::new(7.0) / F::new(144.0) * t51383 - t54293 - t54294 + t54295 / F::new(48.0) - t54297 / F::new(24.0) + t54299 / F::new(48.0) + t54302 + F::new(5.0) / F::new(192.0) * t54303 - F::new(119.0) / F::new(3456.0) * t54305 - t54307 / F::new(48.0) - F::new(35.0) / F::new(576.0) * t51401 + t54310 / F::new(192.0);
    t54312
}
