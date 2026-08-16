//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 684/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk684<F: Float>(t30: F, t265: F, t393: F, t1100: F, t1102: F, t198: F, t336: F, t5023: F, t7177: F, t7181: F, t7193: F, t1996: F, t45: F, t606: F, t7099: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t7194 = piecewise3::<F>(t394, t1102 * t198 * t336 * t7177 - t1100 * t5023 * t7181, t7193);
    let t7199 = piecewise3::<F>(t120, t7099, t1996 * t606 / F::cast_from(2.0_f64) + t7194 * t45 / F::cast_from(2.0_f64));
    (t7194, t7199)
}
