//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 589/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk589<F: Float>(t11661: F, t123: F, t734: F, t11638: F, t11641: F, t11644: F, t11647: F, t11650: F, t11653: F, t11658: F, t1841: F, t1897: F, t2508: F, t3622: F, t3627: F, t3631: F, t650: F) -> (F, F) {
    let t11662 = t11661 * t123;
    let t11663 = t11662 * t734;
    let t11672 = 0.23071578690426672851e-1 * t1897 * t11638 - 0.46143157380853345702e-1 * t2508 * t11641 + 0.76905262301422242837e-2 * t2508 * t11644 - 0.76905262301422242837e-2 * t1897 * t11647 + 0.15381052460284448567e-1 * t2508 * t11650 + 0.76905262301422242837e-2 * t2508 * t11653 - 0.85450291446024714263e-3 * t1841 * t11658 - 0.85450291446024714263e-3 * t1841 * t11663 + 0.20508069947045931423e-1 * t650 * t3627 - 0.10254034973522965712e-1 * t650 * t3631 - 0.30762104920568897135e-1 * t650 * t3622;
    (t11662, t11672)
}
