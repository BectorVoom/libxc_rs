//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1947/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1947<F: Float>(t30: F, t265: F, t393: F, t28271: F, t572: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28273 = F::cast_from(6.0_f64) * t572 * t28271;
    let t28275 = F::cast_from(3.0_f64) * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = F::cast_from(6.0_f64) * t572 * t28277;
    let t28280 = t117 * t28042;
    let t28282 = F::cast_from(3.0_f64) * t572 * t28280;
    let t28998 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t27754);
    let t29005 = piecewise3::<F>(t120, t27408, t7594 * t1469 / F::cast_from(2.0_f64) + t2129 * t4186 / F::cast_from(2.0_f64) + t28998 * t45 / F::cast_from(2.0_f64) + t8161 * t606 / F::cast_from(2.0_f64));
    (t28273, t28275, t28276, t28277, t28279, t28280, t28282, t28998, t29005)
}
