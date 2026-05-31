//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 920/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk920<F: Float>(t30: F, t1469: F, t1996: F, t27408: F, t27755: F, t4186: F, t45: F, t606: F, t7194: F, t7856: F, t33: F, t892: F, t4433: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t27762 = piecewise3::<F>(t120, t27408, t7194 * t1469 / F::cast_from(2.0_f64) + t1996 * t4186 / F::cast_from(2.0_f64) + t27755 * t45 / F::cast_from(2.0_f64) + t7856 * t606 / F::cast_from(2.0_f64));
    let t27763 = t892 * t33;
    let t27764 = t27763 * t4433;
    (t27762, t27764)
}
