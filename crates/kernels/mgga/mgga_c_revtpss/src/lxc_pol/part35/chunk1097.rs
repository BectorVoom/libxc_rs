//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1097/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1097<F: Float>(t33: F, t265: F, t502: F, t30462: F, t1469: F, t2085: F, t30502: F, t57: F, t5825: F, t8059: F, t30470: F, t26405: F, t30122: F, t2047: F, t29532: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t30503 = piecewise3::<F>(t503, F::new(0.0), t30462);
    let t30510 = piecewise3::<F>(t400, t30502, t30503 * t57 / F::new(2.0) - t8059 * t1469 - t2085 * t5825 / F::new(2.0));
    let t30511 = t30470 + t30510;
    let t30513 = t26405 * t30122;
    let t30543 = t2047 * t29532;
    (t30503, t30511, t30513, t30543)
}
