//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2037/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2037<F: Float>(t30: F, t265: F, t393: F, t110792: F, t110839: F, t110158: F, t110196: F, t110711: F, t110745: F, t1469: F, t18281: F, t2078: F, t28523: F, t30463: F, t4186: F, t45: F, t5825: F, t606: F, t7449: F, t8040: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t110840 = t110792 + t110839;
    let t110841 = piecewise3::<F>(t394, F::new(0.0), t110840);
    let t110853 = piecewise3::<F>(t120, t110158 + t110196 + t110711 + t110745, t110841 * t45 / F::new(2.0) + t30463 * t606 / F::new(2.0) + t28523 * t1469 + t8040 * t4186 + t7449 * t5825 / F::new(2.0) + t2078 * t18281 / F::new(2.0));
    (t110840, t110853)
}
