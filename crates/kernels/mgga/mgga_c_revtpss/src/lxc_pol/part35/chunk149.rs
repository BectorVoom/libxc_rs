//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 149/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk149<F: Float>(t33: F, t488: F, t494: F, t460: F, t198: F, t336: F, t424: F, t452: F, t454: F, t265: F, t57: F, t398: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t495 = t488 * t494;
    let t498 = 1.0 + 0.65854491829355115987e0 * t460 * t495;
    let t499 = f64::ln(t498);
    let t502 = t198 * t336 * t499 - t424 + t452 + t454;
    let t503 = t265 < t502;
    let t504 = piecewise3(t503, t502, t265);
    let t507 = piecewise3(t400, t265 * t33 / 2.0, t504 * t57 / 2.0);
    let t508 = t398 + t507;
    (t495, t498, t504, t508)
}
