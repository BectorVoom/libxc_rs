//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 982/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk982<F: Float>(t10732: F, t2662: F, t2661: F, t221: F, t2430: F, t2675: F, t2674: F, t2735: F, t2783: F, t2664: F, t808: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F) -> (F, F, F, F, F, F) {
    let t10733 = t2662 * t10732;
    let t10734 = t2661 * t10733;
    let t10741 = t2675 * t221 * t2430;
    let t10742 = t2674 * t10741;
    let t10744 = t2735 * t2783;
    let t10745 = t808 * t2664;
    let t10746 = t10744 * t10745;
    let t10749 = t2710 * t2713 * t2693;
    let t10756 = 0.72250660161932334527e-3 * t9784 * t810;
    (t10734, t10742, t10744, t10746, t10749, t10756)
}
