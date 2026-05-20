//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2248/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2248<F: Float>(t33: F, t265: F, t502: F, t100973: F, t101021: F, t101064: F, t101105: F, t100927: F, t13312: F, t1469: F, t2003: F, t2258: F, t25792: F, t27822: F, t4186: F, t57: F, t606: F, t7215: F, t7877: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t101107 = t100973 + t101021 + t101064 + t101105;
    let t101108 = piecewise3::<F>(t503, F::new(0.0), t100927);
    let t101120 = piecewise3::<F>(t400, t101107, t101108 * t57 / F::new(2.0) - t27822 * t606 - t7877 * t2258 / F::new(2.0) - t25792 * t1469 / F::new(2.0) - t7215 * t4186 - t2003 * t13312 / F::new(2.0));
    t101120
}
