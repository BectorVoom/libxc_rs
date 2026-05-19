//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 721/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk721<F: Float>(t1984: F, t225: F, t10: F, t670: F, t5311: F, t5314: F, t5316: F, t5318: F, t5324: F, t5326: F, t5328: F, t5330: F, t5332: F, t5337: F, t5339: F, t5341: F, t5345: F, t5348: F, t5350: F, t5354: F) -> (F, F, F) {
    let t5926 = t225 * t1984;
    let t5927 = t10 * t5926;
    let t5929 = F::cast_from(0.32463124087094530131e0_f64) * t670 * t5927;
    let t5930 = -t5311 - t5314 + t5316 - t5318 - t5324 - t5326 - t5328 + t5330 + t5332 + t5337 + t5339 + t5341 + t5345 + t5348 + t5350 + t5354 + t5929;
    (t5926, t5927, t5930)
}
