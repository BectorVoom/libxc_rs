//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 977/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk977<F: Float>(t13945: F, t681: F, t13942: F, t650: F, t270: F, t47420: F, t738: F, t43040: F, t43043: F, t43046: F, t43049: F, t43051: F, t43053: F, t43054: F, t13918: F, t7137: F) -> (F, F) {
    let t47629 = 0.76905262301422242837e-2 * t681 * t13945;
    let t47631 = 0.10254034973522965712e-1 * t650 * t13942;
    let t47634 = 0.76905262301422242837e-2 * t270 * t738 * t47420;
    let t47636 = 0.76905262301422242837e-2 * t681 * t13942;
    let t47639 = -t43040 - t47629 + t47631 - t47634 + t47636 + t43043 + 0.25635087433807414279e-2 * t43046 - t43049 - 0.23071578690426672851e-1 * t43051 - t43053 + t43054;
    let t47640 = t7137 * t13918;
    (t47639, t47640)
}
