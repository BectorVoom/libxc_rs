//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1088/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1088<F: Float>(t21243: F, t21247: F, t21255: F, t21267: F, t21274: F, t21280: F, t21286: F, t21295: F, t21302: F, t21306: F, t21310: F, t21318: F, t21326: F, t21332: F, t21336: F, t21338: F, t21341: F, t21348: F, t21355: F, t21359: F, t21378: F, t21382: F) -> (F, F) {
    let t21705 = -t21243 + t21247 + t21255 - t21267 - t21274 + t21280 - t21286 + t21295 + t21302 - t21306 - t21310;
    let t21708 = t21318 + t21326 + t21332 + t21336 + t21338 - t21341 - t21348 - t21355 + t21359 - t21378 + t21382;
    (t21705, t21708)
}
