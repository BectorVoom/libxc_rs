//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 748/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk748<F: Float>(t1413: F, t8231: F, t25350: F, t492: F, t4265: F, t8220: F, t8224: F, t8212: F, t8216: F, t442: F, t8159: F, t140: F, t299: F, t8227: F, t240: F, t7796: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27180 = t8231 * t1413;
    let t27181 = t27180 * sigma0;
    let t27204 = t25350 * t492;
    let t27270 = t4265 * t8220;
    let t27308 = t4265 * t8224;
    let t27319 = t4265 * t8212;
    let t27321 = t4265 * t8216;
    let t27331 = t8159 * t442;
    let t27355 = t140 * t299 * t8227;
    let t27491 = t240 * t7796;
    (t27181, t27204, t27270, t27308, t27319, t27321, t27331, t27355, t27491)
}
