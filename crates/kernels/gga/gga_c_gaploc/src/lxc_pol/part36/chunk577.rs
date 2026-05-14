//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 577/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk577<F: Float>(t10740: F, t8528: F, t883: F, t2562: F, t943: F, t2549: F, t3437: F, t10716: F, t10720: F, t10722: F, t10733: F, t10735: F, t10739: F, t1897: F, t3434: F, t3452: F, t650: F, t681: F, t9754: F, t9762: F) -> (F,) {
    let t10741 = 0.32043859292259267849e-3 * t10740;
    let t10742 = t883 * t8528;
    let t10743 = t2562 * t10742;
    let t10744 = t943 * t10743;
    let t10745 = 0.32043859292259267849e-3 * t10744;
    let t10746 = t2549 * t3437;
    let t10747 = 0.32043859292259267849e-3 * t10746;
    let t10748 = t10716 + t10720 + 0.76905262301422242837e-2 * t1897 * t10722 - 0.76905262301422242837e-2 * t681 * t3452 + 0.10254034973522965712e-1 * t650 * t3434 - 0.10254034973522965712e-1 * t650 * t3452 + t9754 + t9762 + t10733 + t10735 - t10739 - t10741 - t10745 + t10747;
    (t10748,)
}
