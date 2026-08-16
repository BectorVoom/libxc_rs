//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 164/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk164<F: Float>(t43: F, t605: F, t100: F, t108: F, t101: F, t105: F, t97: F, tau0: F) -> (F, F, F, F, F) {
    let t656 = tau0 * t43;
    let t658 = t605 / F::cast_from(2.0_f64);
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t656 * t101 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t662 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t659;
    (t656, t658, t661, t662, t665)
}
