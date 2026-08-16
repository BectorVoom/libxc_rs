//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 522/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk522<F: Float>(t12: F, t1646: F, t2159: F, t318: F, t319: F, t808: F, t810: F, t201: F, t1281: F, t204: F, t334: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t2163 = piecewise3::<F>(t84, F::cast_from(0.0_f64), t1646);
    let t2167 = piecewise3::<F>(t203, F::cast_from(0.0_f64), t2159 * t319 / F::cast_from(2.0_f64) + t808 * t810 + t318 * t2163 / F::cast_from(2.0_f64));
    let t2168 = t201 * t2167;
    let t2172 = t204 * t1281 * t334;
    (t2163, t2168, t2172)
}
