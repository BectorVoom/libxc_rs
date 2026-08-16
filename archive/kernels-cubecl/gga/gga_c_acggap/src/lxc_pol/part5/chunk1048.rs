//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1048/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1048<F: Float>(t224: F, t4068: F, t1390: F, t709: F, t12930: F, t1549: F, t1554: F, t1558: F, t13263: F, t1545: F, t3379: F, t4291: F) -> (F, F, F, F, F, F, F) {
    let t18217 = t224 * t4068;
    let t18222 = t709 * t1390;
    let t18295 = t12930 * t1549;
    let t18297 = t12930 * t1554;
    let t18299 = t12930 * t1558;
    let t18301 = t13263 * t1545;
    let t18303 = t3379 * t4291;
    (t18217, t18222, t18295, t18297, t18299, t18301, t18303)
}
