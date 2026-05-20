//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1103/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1103<F: Float>(t119974: F, t120157: F, t32247: F, t32283: F, t32192: F, t8583: F, t8584: F, t1413: F, t246: F, t31752: F, t3999: F, t843: F, t8589: F) -> (F, F, F, F, F, F) {
    let t120159 = F::cast_from(0.50779446784275991476e-2_f64) * t120157 * t119974;
    let t120952 = t32247 * t32283;
    let t120956 = t8583 * t8584 * t32192;
    let t120962 = t1413 * t246;
    let t120967 = t31752 * t32192 * t1413;
    let t120975 = t8583 * t8589 * t3999 * t843;
    (t120159, t120952, t120956, t120962, t120967, t120975)
}
