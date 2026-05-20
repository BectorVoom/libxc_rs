//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 700/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk700<F: Float>(t33: F, t265: F, t502: F, t7448: F, t1113: F, t1940: F, t2071: F, t2085: F, t2403: F, t57: F, t606: F, t7200: F, t7207: F, t7428: F, t7432: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7468 = piecewise3::<F>(t503, F::new(0.0), t7448);
    let t7473 = piecewise3::<F>(t400, F::new(3.0) / F::new(2.0) * t2403 * t2071 * t7200 + t1940 * t7428 * t33 / F::new(2.0) - t1940 * t7432 * t7207 / F::new(2.0) + t1940 * t2071 * t1113 / F::new(2.0), -t2085 * t606 / F::new(2.0) + t7468 * t57 / F::new(2.0));
    (t7468, t7473)
}
