//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1396/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1396<F: Float>(t4227: F, t6126: F, t11737: F, t1206: F, t12248: F, t14185: F, t14882: F, t15021: F, t15558: F, t2408: F, t29751: F, t3066: F, t3068: F, t3207: F, t35566: F, t55831: F, t55833: F, t55841: F, t57545: F, t57551: F, t57555: F, t57570: F, t57574: F, t57578: F, t9283: F) -> F {
    let t58854 = t6126 * t4227;
    let t58869 = t55831 + t55833 - t57545 / F::new(24.0) - t2408 * t29751 * t15558 / F::new(12.0) + t57551 / F::new(24.0) + t57555 / F::new(768.0) - t3066 * t35566 * t14882 / F::new(8.0) + t55841 - t57570 / F::new(256.0) - t2408 * t35566 * t15021 / F::new(12.0) - t3066 * t9283 * t58854 * t3068 / F::new(8.0) + t57574 / F::new(768.0) + t57578 / F::new(48.0) - t2408 * t9283 * t14185 * t12248 / F::new(24.0) - t3207 * t9283 * t1206 * t11737 / F::new(16.0);
    t58869
}
