//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1156/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1156<F: Float>(t13942: F, t650: F, t270: F, t47420: F, t738: F, t681: F, t43040: F, t43043: F, t43046: F, t43049: F, t43051: F, t43053: F, t43054: F, t47629: F) -> F {
    let t47631 = F::new(0.10254034973522965712e-1) * t650 * t13942;
    let t47634 = F::new(0.76905262301422242837e-2) * t270 * t738 * t47420;
    let t47636 = F::new(0.76905262301422242837e-2) * t681 * t13942;
    let t47639 = -t43040 - t47629 + t47631 - t47634 + t47636 + t43043 + F::new(0.25635087433807414279e-2) * t43046 - t43049 - F::new(0.23071578690426672851e-1) * t43051 - t43053 + t43054;
    t47639
}
