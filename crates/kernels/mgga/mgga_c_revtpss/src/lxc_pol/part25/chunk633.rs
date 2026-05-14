//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 633/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk633<F: Float>(t30: F, t33: F, t525: F, t605: F, t2257: F, t513: F, t527: F, t1113: F, t3351: F, t516: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3833 = 1.0 / t525;
    let t3834 = t605 * t605;
    let t3840 = piecewise3(t31, 0.0, 4.0 / 9.0 * t3833 * t3834 + 4.0 / 3.0 * t513 * t2257);
    let t3841 = 1.0 / t527;
    let t3842 = t1113 * t1113;
    let t3848 = piecewise3(t34, 0.0, 4.0 / 9.0 * t3841 * t3842 + 4.0 / 3.0 * t516 * t3351);
    let t3850 = (t3840 + t3848) * t162;
    (t3833, t3834, t3841, t3842, t3850)
}
