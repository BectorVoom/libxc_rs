//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 575/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk575<F: Float>(t30: F, t265: F, t502: F, t1940: F, t2072: F, t2078: F, t45: F, t2071: F, t33: F, t2077: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t503 = t265 < t502;
    let t2081 = piecewise3::<F>(t120, t1940 * t2072 / F::new(2.0), t2078 * t45 / F::new(2.0));
    let t2082 = t2071 * t33;
    let t2085 = piecewise3::<F>(t503, F::new(0.0), t2077);
    (t2081, t2082, t2085)
}
