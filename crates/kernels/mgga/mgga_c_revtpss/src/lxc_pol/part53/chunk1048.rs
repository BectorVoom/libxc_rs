//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1048/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1048<F: Float>(t33: F, t265: F, t502: F, t127236: F, t127287: F, t127181: F, t1469: F, t32089: F, t33897: F, t4186: F, t57: F, t606: F, t8553: F, t28189: F, t8568: F, t32099: F, t7898: F, t25082: F, t27153: F, t37110: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t127288 = t127236 + t127287;
    let t127289 = piecewise3(t503, 0.0, t127181);
    let t127296 = piecewise3(t400, t127288, t127289 * t57 / 2.0 - t32089 * t1469 / 2.0 - t33897 * t606 / 2.0 - t8553 * t4186 / 2.0);
    let t127299 = t8568 * t28189;
    let t127302 = 3.0 * t7898 * t32099;
    let t127305 = 6.0 * t25082 * t37110 * t27153;
    (t127296, t127299, t127302, t127305)
}
