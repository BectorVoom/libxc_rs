//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 669/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk669<F: Float>(t12311: F, t734: F, t10645: F, t10647: F, t10685: F, t12291: F, t12294: F, t12297: F, t12306: F, t1841: F, t1897: F, t2508: F, t270: F, t3723: F, t3727: F, t650: F, t681: F, t9635: F, t9651: F, t9654: F) -> F {
    let t12312 = t12311 * t734;
    let t12315 = -t10645 - t10647 + F::new(0.76905262301422242837e-2) * t2508 * t12291 + F::new(0.15381052460284448567e-1) * t2508 * t12294 + F::new(0.76905262301422242837e-2) * t1897 * t12297 - t9635 - t9651 + t9654 + F::new(0.10254034973522965712e-1) * t650 * t3723 + F::new(0.76905262301422242837e-2) * t681 * t3723 + F::new(0.76905262301422242837e-2) * t270 * t12306 - F::new(0.10254034973522965712e-1) * t650 * t3727 + t10685 - F::new(0.85450291446024714263e-3) * t1841 * t12312;
    t12315
}
