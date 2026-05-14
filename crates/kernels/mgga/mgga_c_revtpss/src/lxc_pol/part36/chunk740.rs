//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 740/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk740<F: Float>(t5: F, t30: F, t265: F, t393: F, t1923: F, t2123: F, t7566: F, t7702: F, t7706: F, t7709: F, t8144: F, t8147: F, t117: F, t1518: F, t2163: F, t7855: F, t1469: F, t2129: F, t45: F, t7794: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8151 = piecewise3(t8, 0.0, -t7702 * t2123 / 6.0 + 5.0 / 6.0 * t7566 * t7706 + t7709 * t2123 / 3.0 - t1923 * t8144 / 6.0 - t1923 * t8147 / 6.0);
    let t8152 = t8151 * t117;
    let t8158 = t2163 * t1518;
    let t8161 = piecewise3(t394, 0.0, t7855);
    let t8166 = piecewise3(t120, t7794, t2129 * t1469 / 2.0 + t8161 * t45 / 2.0);
    (t8151, t8152, t8158, t8161, t8166)
}
