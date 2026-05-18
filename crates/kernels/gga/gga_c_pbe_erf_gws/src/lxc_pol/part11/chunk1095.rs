//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1095/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1095<F: Float>(t47556: F, t7062: F, t7069: F, t23109: F, t30666: F, t40321: F, t22996: F, t47545: F, t47546: F, t47547: F, t47548: F, t47552: F, t47554: F, t47555: F) -> (F, F, F, F, F) {
    let t47559 = F::new(16.0) / F::new(9.0) * t7062 * t7069 * t47556;
    let t47560 = F::new(128.0) / F::new(405.0) * t23109;
    let t47561 = F::new(16.0) / F::new(45.0) * t30666;
    let t47562 = F::new(32.0) / F::new(27.0) * t40321;
    let t47563 = F::new(0.14e-19) * t22996 - t47545 + t47546 + t47547 - t47548 + t47552 + t47554 - t47555 - t47559 + t47560 + t47561 + t47562;
    (t47559, t47560, t47561, t47562, t47563)
}
