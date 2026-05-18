//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 403/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk403<F: Float>(t33: F, t265: F, t502: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t1298: F, t1300: F, t198: F, t336: F, t895: F, t1113: F, t504: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t1304 = piecewise3::<f64>(t503, t1298 * t1300 * t198 * t336 - t1128 + t1153 + t1193 + t1195 - t1200, t895);
    let t1309 = piecewise3::<f64>(t400, t265 * t1113 / F::new(2.0) + t895 * t33 / F::new(2.0), t1304 * t57 / F::new(2.0) - t504 * t606 / F::new(2.0));
    (t1304, t1309)
}
