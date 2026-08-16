//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2845/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845<F: Float>(t45: F, t221: F, t23177: F, t2484: F, t2485: F, t14325: F, t23216: F, t1469: F, t4401: F, t61303: F, t14401: F, t14404: F, t18272: F, t18281: F, t19680: F, t22671: F, t22688: F, t2375: F, t39825: F, t4186: F, t4377: F, t5825: F, t606: F, t76397: F, t78: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t76887 = t2484 * t2485 * t221 * t23177;
    let t76890 = F::cast_from(36.0_f64) * t14325 * t23216;
    let t76892 = t4401 * t61303 * t1469;
    let t76893 = F::cast_from(36.0_f64) * t76892;
    let t76911 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39825 * t22688 * t606 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t18272 * t4186 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t14401 * t19680 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14404 * t5825 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4377 * t18281 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2375 * t22671 * t606 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t76397);
    (t76887, t76890, t76893, t76911)
}
