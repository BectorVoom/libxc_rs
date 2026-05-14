//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 608/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk608<F: Float>(t1692: F, t3222: F, t12380: F, t713: F, t928: F, t12411: F, t295: F, t10007: F, t935: F, t9438: F, t825: F, t10012: F, t2684: F, t2321: F, t3371: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12566 = t1692 * t3222;
    let t12568 = t713 * t12380;
    let t12569 = t12568 * t928;
    let t12580 = t295 * t12411;
    let t12691 = t10007 * t935;
    let t12692 = t9438 * t12691;
    let t12693 = t825 * t12692;
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12770 = t3371 * t2321;
    let t12771 = t882 * t12770;
    (t12566, t12568, t12569, t12580, t12691, t12692, t12693, t12704, t12705, t12706, t12770, t12771)
}
