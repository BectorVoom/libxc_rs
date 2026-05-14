//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1093/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1093<F: Float>(t1458: F, t9481: F, t2726: F, t4169: F, t1299: F, t1486: F, t394: F, t4208: F) -> (F, F, F, F) {
    let t32226 = t9481 * t1458;
    let t32229 = t2726 * t4169;
    let t32255 = t1486 * t1299;
    let t32260 = t4208 * t394;
    (t32226, t32229, t32255, t32260)
}
