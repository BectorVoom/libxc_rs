//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 985/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk985<F: Float>(t30: F, t265: F, t393: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, t5273: F, t7617: F, t5291: F, t7616: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28998 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t27754);
    let t29005 = piecewise3::<F>(t120, t27408, t7594 * t1469 / F::cast_from(2.0_f64) + t2129 * t4186 / F::cast_from(2.0_f64) + t28998 * t45 / F::cast_from(2.0_f64) + t8161 * t606 / F::cast_from(2.0_f64));
    let t29010 = t5273 * t7617;
    let t29019 = t7616 * t5291;
    (t29005, t29010, t29019)
}
