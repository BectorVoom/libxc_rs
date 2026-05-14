//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 599/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk599<F: Float>(t1961: F, t1965: F, t1964: F, t760: F, t755: F, t1973: F) -> (F, F, F, F) {
    let t5368 = t1961 * t1965;
    let t5371 = t1964 * t760;
    let t5372 = 1.0 / t5371;
    let t5373 = t755 * t5372;
    let t5374 = t1973 * t1973;
    (t5368, t5372, t5373, t5374)
}
