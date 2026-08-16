//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 702/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk702(t10615: f64, t12968: f64, t12792: f64, t189: f64, t188: f64, t600: f64, t568: f64, t12793: f64, t531: f64, t569: f64, t9448: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12969 = t10615 * t12968;
    let t12970 = 0.89376224879626066675e-1_f64 * t12969;
    let t12971 = t189 * t12792;
    let t12972 = t188 * t12971;
    let t12975 = t600 * t12792;
    let t12976 = t568 * t12975;
    let t12979 = t531 * t12793;
    let t12982 = t569 * t12792;
    let t12983 = t568 * t12982;
    let t12986 = t9448 * t986;
    (t12970, t12971, t12972, t12975, t12976, t12979, t12982, t12983, t12986)
}
