//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 943/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk943<F: Float>(t3722: F, t779: F, t12214: F, t2580: F, t12259: F, t1901: F, t12161: F, t169: F, t299: F, t706: F, t12250: F, t123: F, t734: F, t10645: F, t10647: F, t10685: F, t1841: F, t1897: F, t2508: F, t270: F, t3723: F, t3727: F, t650: F, t681: F, t9635: F, t9651: F, t9654: F) -> (F, F, F, F, F, F, F) {
    let t12291 = t779 * t3722;
    let t12294 = t2580 * t12214;
    let t12297 = t1901 * t12259;
    let t12305 = t12161 * t169 * t299;
    let t12306 = t706 * t12305;
    let t12311 = t12250 * t123;
    let t12312 = t12311 * t734;
    let t12315 = -t10645 - t10647 + 0.76905262301422242837e-2 * t2508 * t12291 + 0.15381052460284448567e-1 * t2508 * t12294 + 0.76905262301422242837e-2 * t1897 * t12297 - t9635 - t9651 + t9654 + 0.10254034973522965712e-1 * t650 * t3723 + 0.76905262301422242837e-2 * t681 * t3723 + 0.76905262301422242837e-2 * t270 * t12306 - 0.10254034973522965712e-1 * t650 * t3727 + t10685 - 0.85450291446024714263e-3 * t1841 * t12312;
    (t12291, t12294, t12297, t12305, t12306, t12312, t12315)
}
