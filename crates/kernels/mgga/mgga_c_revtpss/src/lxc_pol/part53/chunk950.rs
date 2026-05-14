//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 950/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk950<F: Float>(t5: F, t8142: F, t8435: F, t2247: F, t32798: F, t32802: F, t33621: F, t34173: F, t34177: F, t34181: F, t34402: F, t8623: F, t8737: F, t117: F, t118: F, t1843: F, t1932: F, t2163: F, t33600: F, t33603: F, t33605: F, t33650: F, t33654: F, t34383: F, t34394: F, t34400: F, t34401: F, t508: F, t7725: F, t8233: F, t8741: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t34409 = t8435 * t8142;
    let t34410 = t2247 * t34409;
    let t34418 = piecewise3(t8, 0.0, 5.0 / 144.0 * t34402 * t8623 - 5.0 / 24.0 * t32798 * t34173 - 5.0 / 36.0 * t32802 * t34177 + 5.0 / 144.0 * t34410 * t8623 + 5.0 / 72.0 * t8737 * t34181 + 5.0 / 144.0 * t8737 * t33621);
    let t34419 = t34418 * t117;
    let t34422 = -t118 * t34394 - t1843 * t8741 - t1932 * t8233 - t2163 * t7725 - t34419 * t508 - 2.0 * t33600 - 2.0 * t33603 - 2.0 * t33605 - t33650 - t33654 - 2.0 * t34383 + t34400 + t34401;
    (t34409, t34410, t34418, t34419, t34422)
}
