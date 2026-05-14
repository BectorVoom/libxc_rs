//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 941/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk941<F: Float>(t2681: F, t820: F, t849: F, t857: F, t240: F, t2719: F, t2735: F, t2783: F, t2664: F, t808: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F, t9789: F) -> (F, F, F, F, F, F, F, F) {
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    let t10744 = t2735 * t2783;
    let t10745 = t808 * t2664;
    let t10746 = t10744 * t10745;
    let t10749 = t2710 * t2713 * t2693;
    let t10756 = 0.72250660161932334527e-3 * t9784 * t810;
    let t10758 = 0.11294745624363664198e-6 * t9789 * t810;
    (t10722, t10723, t10726, t10744, t10746, t10749, t10756, t10758)
}
