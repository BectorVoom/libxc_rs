//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 667/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk667<F: Float>(t12166: F, t738: F, t12255: F, t740: F, t3732: F, t702: F, t10631: F, t10634: F, t10638: F, t10642: F, t1897: F, t2508: F, t270: F, t3727: F, t681: F, t9618: F, t9620: F, t9622: F, t9627: F, t9629: F, t9632: F) -> F {
    let t12281 = t738 * t12166;
    let t12284 = t12255 * t740;
    let t12287 = t3732 * t702;
    let t12290 = -F::cast_from(0.76905262301422242837e-2_f64) * t681 * t3727 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t12281 + t9618 - t9620 - t9622 - t9627 + t9629 + t9632 - t10631 + t10634 - t10638 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t12284 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t12287 - t10642;
    t12290
}
