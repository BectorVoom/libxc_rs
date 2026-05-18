//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 829/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk829<F: Float>(t1416: F, t9779: F, t124: F, t212: F, t2237: F, t800: F, t1376: F, t123: F, t125: F, t2452: F, t9720: F, t235: F, t4086: F) -> (F, F, F, F, F, F) {
    let t9780 = t9779 * t1416;
    let t9784 = t800 * t124 * t2237 * t212;
    let t9786 = F::new(0.72250660161932334527e-3) * t9784 * t1376;
    let t9789 = t123 * t125 * t9720 * t2452;
    let t9791 = F::new(0.11294745624363664198e-6) * t9789 * t1376;
    let t9792 = t4086 * t235;
    (t9780, t9784, t9786, t9789, t9791, t9792)
}
