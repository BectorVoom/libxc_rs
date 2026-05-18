//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 831/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk831<F: Float>(t4265: F, t8216: F, t442: F, t8159: F, t140: F, t299: F, t8227: F, t240: F, t7796: F, t1528: F, t8344: F, t4463: F, t8365: F) -> (F, F, F, F, F, F) {
    let t27321 = t4265 * t8216;
    let t27331 = t8159 * t442;
    let t27355 = t140 * t299 * t8227;
    let t27491 = t240 * t7796;
    let t27516 = t8344 * t1528;
    let t27584 = t8365 * t4463;
    (t27321, t27331, t27355, t27491, t27516, t27584)
}
