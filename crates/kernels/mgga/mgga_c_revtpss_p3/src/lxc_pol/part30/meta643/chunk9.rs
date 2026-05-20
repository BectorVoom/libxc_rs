//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2258/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2258<F: Float>(t33: F, t265: F, t502: F, t100927: F, t105696: F, t101107: F, t13312: F, t1469: F, t2159: F, t2258: F, t27048: F, t29329: F, t4186: F, t57: F, t606: F, t7677: F, t8227: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t105697 = piecewise3::<F>(t503, t105696, t100927);
    let t105709 = piecewise3::<F>(t400, t101107, t105697 * t57 / F::new(2.0) - t29329 * t606 - t8227 * t2258 / F::new(2.0) - t27048 * t1469 / F::new(2.0) - t7677 * t4186 - t2159 * t13312 / F::new(2.0));
    t105709
}
