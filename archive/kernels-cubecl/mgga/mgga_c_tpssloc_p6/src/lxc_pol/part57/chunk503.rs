//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 503/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk503<F: Float>(t28: F, t265: F, t504: F, t5669: F, t6278: F, t1409: F, t1534: F, t1649: F, t1768: F, t506: F, t52: F, t5398: F, t5966: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t6279 = piecewise3::<F>(t505, t6278, t5669);
    let t6286 = piecewise3::<F>(t401, t5669 * t28 / F::cast_from(2.0_f64) + t1534 * t1649 + t265 * t5966 / F::cast_from(2.0_f64), t6279 * t52 / F::cast_from(2.0_f64) - t1768 * t1409 - t506 * t5398 / F::cast_from(2.0_f64));
    t6286
}
