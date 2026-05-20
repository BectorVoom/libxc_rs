//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2241/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2241<F: Float>(t30: F, t265: F, t393: F, t100882: F, t100926: F, t100833: F, t13312: F, t1469: F, t1996: F, t2258: F, t25744: F, t27755: F, t4186: F, t45: F, t606: F, t7194: F, t7856: F, t99565: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t100927 = t100882 + t100926;
    let t100928 = piecewise3::<F>(t394, t100833, t100927);
    let t100940 = piecewise3::<F>(t120, t99565, t100928 * t45 / F::new(2.0) + t27755 * t606 + t7856 * t2258 / F::new(2.0) + t25744 * t1469 / F::new(2.0) + t7194 * t4186 + t1996 * t13312 / F::new(2.0));
    (t100927, t100940)
}
