//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1226/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1226<F: Float>(t33: F, t265: F, t502: F, t128061: F, t128097: F, t128121: F, t128150: F, t128183: F, t1469: F, t32569: F, t34161: F, t4186: F, t57: F, t606: F, t8682: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t128186 = piecewise3::<F>(t503, F::new(0.0), t128061);
    let t128193 = piecewise3::<F>(t400, t128097 + t128121 + t128150 + t128183, t128186 * t57 / F::new(2.0) - t32569 * t1469 / F::new(2.0) - t34161 * t606 / F::new(2.0) - t8682 * t4186 / F::new(2.0));
    t128193
}
