//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 876/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk876<F: Float>(t1184: F, t12747: F, t1190: F, t3378: F, t3430: F, t1177: F, t12727: F, t1017: F, t1459: F, t384: F, t398: F, t879: F) -> (F, F, F, F, F, F) {
    let t12748 = t12747 * t1184;
    let t12750 = t12747 * t1190;
    let t12752 = t3378 * t3430;
    let t12753 = t12752 * t1177;
    let t12755 = t12727 * t1184;
    let t12762 = t384 * t398 * t1459 * t1017 * t879;
    (t12748, t12750, t12752, t12753, t12755, t12762)
}
