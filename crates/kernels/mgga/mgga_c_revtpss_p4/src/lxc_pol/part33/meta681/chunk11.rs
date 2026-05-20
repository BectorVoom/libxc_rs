//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2232/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2232<F: Float>(t30: F, t265: F, t393: F, t107868: F, t106638: F, t1469: F, t18281: F, t2129: F, t28998: F, t30727: F, t4186: F, t45: F, t5825: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t111797 = piecewise3::<F>(t394, F::new(0.0), t107868);
    let t111809 = piecewise3::<F>(t120, t106638, t111797 * t45 / F::new(2.0) + t30727 * t606 / F::new(2.0) + t28998 * t1469 + t8161 * t4186 + t7594 * t5825 / F::new(2.0) + t2129 * t18281 / F::new(2.0));
    t111809
}
