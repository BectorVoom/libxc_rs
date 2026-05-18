//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1198/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1198<F: Float>(t21033: F, t858: F, t866: F, t867: F, t19553: F, t21269: F, t21274: F, t21280: F, t21286: F, t21287: F, t21295: F, t21302: F, t21306: F, t2343: F, t2345: F, t6220: F, t6308: F, t6555: F, t904: F, t916: F, t929: F, t933: F) -> (F, F) {
    let t21310 = t866 * t867 * t858 * t21033 / F::new(96.0);
    let t21311 = F::new(595.0) / F::new(576.0) * t21269 - t21274 - t929 * t933 * t904 * t19553 / F::new(768.0) + t21280 + t2343 * t2345 * t6308 * t6220 / F::new(64.0) - t21286 - F::new(3.0) / F::new(64.0) * t6555 * t916 * t904 * t21287 + t21295 + t21302 - t21306 - t21310;
    (t21310, t21311)
}
