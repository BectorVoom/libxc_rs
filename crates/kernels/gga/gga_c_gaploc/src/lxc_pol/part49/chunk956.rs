//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 956/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk956<F: Float>(t43716: F, t43719: F, t43721: F, t43723: F, t43726: F, t43729: F, t43731: F, t43735: F, t43737: F, t43740: F, t43743: F, t43746: F, t47294: F, t7584: F, t7585: F, t10930: F, t10931: F, t47243: F) -> (F, F, F) {
    let t47354 = -t43716 + t43719 + t43721 + t43723 + t43726 + t43729 + 0.71500979903700853338e0 * t43731 - t43735 + t43737 - t43740 - t43743 - t43746;
    let t47357 = t7584 * t7585 * t47294;
    let t47360 = t10930 * t10931 * t47243;
    (t47354, t47357, t47360)
}
