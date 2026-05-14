//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 506/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk506<F: Float>(t739: F, t9688: F, t738: F, t169: F, t299: F, t706: F, t3266: F, t702: F, t2532: F, t954: F, t1897: F, t2508: F, t270: F, t3212: F, t3237: F, t3244: F, t3252: F, t3256: F, t650: F, t681: F) -> (F, F) {
    let t9689 = t739 * t9688;
    let t9690 = t738 * t9689;
    let t9698 = t9688 * t169 * t299;
    let t9699 = t706 * t9698;
    let t9712 = t3266 * t702;
    let t9715 = t954 * t2532;
    let t9718 = -0.76905262301422242837e-2 * t270 * t9690 + 0.76905262301422242837e-2 * t681 * t3212 + 0.76905262301422242837e-2 * t681 * t3237 + 0.76905262301422242837e-2 * t270 * t9699 - 0.10254034973522965712e-1 * t650 * t3256 + 0.20508069947045931423e-1 * t650 * t3252 - 0.30762104920568897135e-1 * t650 * t3244 + 0.10254034973522965712e-1 * t650 * t3212 + 0.10254034973522965712e-1 * t650 * t3237 - 0.76905262301422242837e-2 * t1897 * t9712 + 0.15381052460284448567e-1 * t2508 * t9715;
    (t9689, t9718)
}
