//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1001/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1001(t1617: f64, t1841: f64, t5519: f64, t732: f64, t265: f64, t266: f64, t837: f64, t17408: f64, t17410: f64, t17412: f64, t17414: f64, t17416: f64, t17420: f64, t17425: f64, t17430: f64) -> f64 {
    let t18274 = t1841 * t1617;
    let t18276 = t732 * t5519;
    let t18280 = 56.0_f64 / 1215.0_f64 * t265 * t266 * t837;
    let t18281 = -t17408 - t17410 + t17412 + t17414 + 4.0_f64 / 45.0_f64 * t18274 - 32.0_f64 / 405.0_f64 * t18276 + t18280 + t17416 - t17420 - t17425 - t17430;
    t18281
}
