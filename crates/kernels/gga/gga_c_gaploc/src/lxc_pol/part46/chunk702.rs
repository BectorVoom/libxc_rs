//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 702/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk702<F: Float>(t10615: F, t12968: F, t12792: F, t189: F, t188: F, t600: F, t568: F, t12793: F, t531: F, t569: F, t9448: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12969 = t10615 * t12968;
    let t12970 = F::new(0.89376224879626066675e-1) * t12969;
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
