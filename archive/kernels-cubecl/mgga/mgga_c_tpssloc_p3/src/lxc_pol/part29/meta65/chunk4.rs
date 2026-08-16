//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 444/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk444<F: Float>(t28: F, t265: F, t504: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t1254: F, t1256: F, t193: F, t336: F, t873: F, t1081: F, t506: F, t52: F, t607: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t1260 = piecewise3::<F>(t505, t1254 * t1256 * t193 * t336 - t1096 + t1121 + t1161 + t1163 - t1168, t873);
    let t1265 = piecewise3::<F>(t401, t265 * t1081 / F::cast_from(2.0_f64) + t873 * t28 / F::cast_from(2.0_f64), t1260 * t52 / F::cast_from(2.0_f64) - t506 * t607 / F::cast_from(2.0_f64));
    (t1260, t1265)
}
