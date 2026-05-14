//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 888/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk888<F: Float>(t1211: F, t6573: F, t1774: F, t1828: F, t1277: F, t3579: F, t5044: F, t6423: F, t6427: F, t6431: F) -> (F, F, F) {
    let t6574 = t1211 * t6573;
    let t6579 = t1774 * t1828;
    let t6580 = t1277 * t6579;
    let t6587 = t3579 - 0.9877777777777777778e-2 * t5044 - 0.9877777777777777778e-2 * t6423 + 0.29633333333333333334e-1 * t6427 + 0.14816666666666666667e-1 * t6431;
    (t6574, t6580, t6587)
}
