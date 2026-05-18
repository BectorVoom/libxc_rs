//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 891/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk891<F: Float>(t810: F, t9784: F, t9789: F, t235: F, t2783: F, t2453: F, t2664: F, t9794: F, t125: F, t2430: F, t2747: F, t837: F) -> (F, F, F, F, F) {
    let t10756 = F::new(0.72250660161932334527e-3) * t9784 * t810;
    let t10758 = F::new(0.11294745624363664198e-6) * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10761 = t9794 * t2664;
    let t10762 = t10760 * t10761;
    let t10764 = t125 * t2430;
    let t10766 = t2747 * t10764 * t837;
    (t10756, t10758, t10761, t10762, t10766)
}
