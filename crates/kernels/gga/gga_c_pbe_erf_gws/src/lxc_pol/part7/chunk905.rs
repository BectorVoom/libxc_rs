//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 905/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk905<F: Float>(t265: F, t266: F, t837: F, t17408: F, t17410: F, t17412: F, t17414: F, t17416: F, t17420: F, t17425: F, t17430: F, t18274: F, t18276: F, t17432: F, t17434: F, t17436: F, t17439: F, t17443: F, t17448: F, t17450: F, t17452: F, t17456: F, t17461: F, t17463: F, t17465: F) -> (F, F) {
    let t18280 = 56.0 / 1215.0 * t265 * t266 * t837;
    let t18281 = -t17408 - t17410 + t17412 + t17414 + 4.0 / 45.0 * t18274 - 32.0 / 405.0 * t18276 + t18280 + t17416 - t17420 - t17425 - t17430;
    let t18282 = t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452 - t17456 - t17461 + t17463 + t17465;
    (t18281, t18282)
}
