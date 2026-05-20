//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2038/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2038<F: Float>(t33: F, t265: F, t502: F, t103707: F, t103750: F, t103778: F, t103817: F, t103853: F, t13312: F, t1469: F, t2085: F, t2258: F, t26666: F, t28578: F, t4186: F, t57: F, t606: F, t7468: F, t8059: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t103856 = piecewise3::<F>(t503, F::new(0.0), t103707);
    let t103868 = piecewise3::<F>(t400, t103750 + t103778 + t103817 + t103853, t103856 * t57 / F::new(2.0) - t28578 * t606 - t8059 * t2258 / F::new(2.0) - t26666 * t1469 / F::new(2.0) - t7468 * t4186 - t2085 * t13312 / F::new(2.0));
    t103868
}
