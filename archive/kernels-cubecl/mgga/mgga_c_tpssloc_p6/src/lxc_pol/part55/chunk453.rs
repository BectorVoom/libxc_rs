//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 453/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk453<F: Float>(t28: F, t265: F, t504: F, t1238: F, t2121: F, t2124: F, t2145: F, t2155: F, t498: F, t1256: F, t193: F, t1964: F, t336: F, t1971: F, t52: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t2157 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t2124 + t2145 * t498 - t1238 * t2155;
    let t2161 = piecewise3::<F>(t505, t193 * t336 * t2157 * t1256, t1964);
    let t2164 = piecewise3::<F>(t401, t1971, t2161 * t52 / F::cast_from(2.0_f64));
    (t2157, t2161, t2164)
}
