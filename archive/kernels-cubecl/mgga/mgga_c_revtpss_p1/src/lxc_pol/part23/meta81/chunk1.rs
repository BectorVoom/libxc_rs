//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 564/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk564<F: Float>(t30: F, t1468: F, t1469: F, t1587: F, t1704: F, t265: F, t395: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t1709 = piecewise3::<F>(t120, t265 * t1468 / F::cast_from(2.0_f64) + t1587 * t30 / F::cast_from(2.0_f64), t395 * t1469 / F::cast_from(2.0_f64) + t1704 * t45 / F::cast_from(2.0_f64));
    let t1711 = -t1468;
    (t1709, t1711)
}
