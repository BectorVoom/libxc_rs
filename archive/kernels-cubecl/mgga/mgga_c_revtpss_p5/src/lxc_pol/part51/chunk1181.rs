//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1181/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1181<F: Float>(t33: F, t265: F, t502: F, t127236: F, t127287: F, t127181: F, t1469: F, t32089: F, t33897: F, t4186: F, t57: F, t606: F, t8553: F, t28189: F, t8568: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t127288 = t127236 + t127287;
    let t127289 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t127181);
    let t127296 = piecewise3::<F>(t400, t127288, t127289 * t57 / F::cast_from(2.0_f64) - t32089 * t1469 / F::cast_from(2.0_f64) - t33897 * t606 / F::cast_from(2.0_f64) - t8553 * t4186 / F::cast_from(2.0_f64));
    let t127299 = t8568 * t28189;
    (t127296, t127299)
}
