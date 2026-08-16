//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 453/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk453(t28: f64, t265: f64, t504: f64, t1238: f64, t2121: f64, t2124: f64, t2145: f64, t2155: f64, t498: f64, t1256: f64, t193: f64, t1964: f64, t336: f64, t1971: f64, t52: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t2157 = 0.82246703342411321825e-2_f64 * t2121 * t2124 + t2145 * t498 - t1238 * t2155;
    let t2161 = piecewise3(t505, t193 * t336 * t2157 * t1256, t1964);
    let t2164 = piecewise3(t401, t1971, t2161 * t52 / 2.0_f64);
    (t2157, t2161, t2164)
}
