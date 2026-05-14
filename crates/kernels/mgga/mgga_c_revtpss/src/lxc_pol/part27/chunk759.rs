//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 759/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk759<F: Float>(t212: F, t225: F, t596: F, t816: F, t3995: F, t1408: F, t2681: F, t820: F, t1416: F, t124: F, t2237: F, t800: F, t1376: F, t123: F, t125: F, t2452: F, t9720: F) -> (F, F, F, F, F, F) {
    let t9775 = t816 * t596 * t212 * t225;
    let t9776 = t9775 * t3995;
    let t9779 = t820 * t1408 * t2681;
    let t9780 = t9779 * t1416;
    let t9784 = t800 * t124 * t2237 * t212;
    let t9786 = 0.72250660161932334527e-3 * t9784 * t1376;
    let t9789 = t123 * t125 * t9720 * t2452;
    (t9775, t9776, t9780, t9784, t9786, t9789)
}
