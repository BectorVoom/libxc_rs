//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1921/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1921<F: Float>(t33: F, t1469: F, t2159: F, t27821: F, t29329: F, t4186: F, t57: F, t606: F, t7677: F, t8227: F, t29005: F, t118: F, t1502: F, t2163: F, t27116: F, t27118: F, t27120: F, t27122: F, t27125: F, t27128: F, t27130: F, t27132: F, t27134: F, t4246: F, t4293: F, t4297: F, t7586: F, t7683: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t29336 = piecewise3::<F>(t400, t27821, -t7677 * t1469 / F::cast_from(2.0_f64) - t2159 * t4186 / F::cast_from(2.0_f64) + t29329 * t57 / F::cast_from(2.0_f64) - t8227 * t606 / F::cast_from(2.0_f64));
    let t29337 = t29005 + t29336;
    let t29343 = -t118 * t29337 - t1502 * t7683 - t2163 * t4246 - F::cast_from(2.0_f64) * t4293 * t7586 - F::cast_from(2.0_f64) * t4297 * t7586 - t27116 - t27118 - t27120 - t27122 - t27125 - t27128 - t27130 - t27132 - t27134;
    (t29337, t29343)
}
