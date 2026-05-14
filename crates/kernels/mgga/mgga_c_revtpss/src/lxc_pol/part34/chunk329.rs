//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 329/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk329<F: Float>(t45: F, t57: F, t1469: F, t190: F, t706: F, t78: F, t81: F, t150: F, t162: F, t187: F, t766: F, t770: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1522 = t190 * t1469;
    let t1524 = 4.0 * t706 * t1522;
    let t1527 = piecewise3(t151, 0.0, 4.0 / 3.0 * t78 * t1469);
    let t1530 = piecewise3(t155, 0.0, -4.0 / 3.0 * t81 * t1469);
    let t1531 = t1527 + t1530;
    let t1532 = t150 * t1531;
    let t1533 = t1532 * t190;
    let t1534 = t1531 * t162;
    let t1536 = 0.19751673498613801407e-1 * t1534 * t187;
    let t1539 = piecewise3(t151, 0.0, 2.0 / 3.0 * t766 * t1469);
    let t1542 = piecewise3(t155, 0.0, -2.0 / 3.0 * t770 * t1469);
    let t1544 = t1539 / 2.0 + t1542 / 2.0;
    (t1522, t1524, t1531, t1532, t1533, t1534, t1536, t1544)
}
