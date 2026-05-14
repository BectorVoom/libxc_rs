//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1011/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1011<F: Float>(t30: F, t1468: F, t9335: F, t2: F, t3833: F, t580: F, t605: F, t22: F, t2257: F, t3834: F, t513: F, t5549: F, t5552: F, t1711: F, t9350: F, t3841: F, t1113: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t13550 = t9335 * t1468;
    let t13553 = t3833 * t2;
    let t13554 = t580 * t605;
    let t13564 = piecewise3(t31, 0.0, -8.0 / 27.0 * t13550 * t3834 + 16.0 / 9.0 * t13553 * t13554 + 4.0 / 9.0 * t5549 * t2257 + 8.0 / 3.0 * t513 * t580 - 8.0 * t5552 * t22);
    let t13565 = t9350 * t1711;
    let t13568 = t3841 * t2;
    let t13569 = t580 * t1113;
    (t13554, t13564, t13565, t13568, t13569)
}
