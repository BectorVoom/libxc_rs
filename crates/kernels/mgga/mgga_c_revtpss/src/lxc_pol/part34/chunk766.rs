//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 766/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk766<F: Float>(t2482: F, t596: F, t849: F, t2681: F, t820: F, t240: F, t2719: F, t2735: F, t2783: F, t810: F, t9784: F, t9789: F, t235: F, t2453: F, t2475: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t10716 = t2482 * t849 * t596;
    let t10722 = t820 * t849 * t2681;
    let t10726 = t2719 * t240;
    let t10744 = t2735 * t2783;
    let t10756 = 0.72250660161932334527e-3 * t9784 * t810;
    let t10758 = 0.11294745624363664198e-6 * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10769 = t2475 * t72;
    (t10716, t10722, t10726, t10744, t10756, t10758, t10760, t10769)
}
