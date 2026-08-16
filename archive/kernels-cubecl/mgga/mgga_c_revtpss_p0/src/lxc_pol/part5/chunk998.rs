//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 998/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk998<F: Float>(t2735: F, t2783: F, t2664: F, t808: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F, t9789: F, t235: F, t2453: F) -> (F, F, F, F, F, F) {
    let t10744 = t2735 * t2783;
    let t10745 = t808 * t2664;
    let t10746 = t10744 * t10745;
    let t10749 = t2710 * t2713 * t2693;
    let t10756 = F::cast_from(0.72250660161932334527e-3_f64) * t9784 * t810;
    let t10758 = F::cast_from(0.11294745624363664198e-6_f64) * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    (t10744, t10746, t10749, t10756, t10758, t10760)
}
