//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1001/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1001<F: Float>(t1617: F, t1841: F, t5519: F, t732: F, t265: F, t266: F, t837: F, t17408: F, t17410: F, t17412: F, t17414: F, t17416: F, t17420: F, t17425: F, t17430: F) -> F {
    let t18274 = t1841 * t1617;
    let t18276 = t732 * t5519;
    let t18280 = F::cast_from(56.0_f64) / F::cast_from(1215.0_f64) * t265 * t266 * t837;
    let t18281 = -t17408 - t17410 + t17412 + t17414 + F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t18274 - F::cast_from(32.0_f64) / F::cast_from(405.0_f64) * t18276 + t18280 + t17416 - t17420 - t17425 - t17430;
    t18281
}
