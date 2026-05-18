//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 701/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk701<F: Float>(t7923: F, t7924: F, t1394: F, t1458: F, t2243: F, t303: F, t1466: F, t541: F) -> (F, F, F, F, F) {
    let t7925 = t7923 * t7924;
    let t7926 = t1394 * t7925;
    let t7928 = t1458 * t2243;
    let t7929 = t303 * t7928;
    let t7931 = t541 * t1466;
    (t7925, t7926, t7928, t7929, t7931)
}
