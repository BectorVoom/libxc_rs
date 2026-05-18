//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 366/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk366<F: Float>(t1457: F, t2950: F, t1035: F, t769: F, t2925: F, t314: F, t313: F, t2963: F, t531: F, t808: F, t568: F, t836: F) -> (F, F, F, F, F, F, F) {
    let t3043 = t1457 * t2950;
    let t3046 = t769 * t1035;
    let t3049 = t314 * t2925;
    let t3050 = t313 * t3049;
    let t3055 = t531 * t2963;
    let t3060 = t808 * t2925;
    let t3061 = t568 * t3060;
    let t3066 = t836 * t2925;
    (t3043, t3046, t3049, t3050, t3055, t3061, t3066)
}
