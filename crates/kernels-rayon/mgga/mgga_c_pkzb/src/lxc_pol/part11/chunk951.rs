//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 951/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk951(t24: f64, t10369: f64, t10374: f64, t8742: f64, t1263: f64, t1265: f64, t3289: f64, t3293: f64, t3940: f64, t3944: f64, t422: f64, t423: f64, t960: f64, t962: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t10375 = t10369 + t10374;
    let t10384 = piecewise3(t90, 0.0_f64, t8742);
    let t10388 = piecewise3(t332, 0.0_f64, t10375 * t423 / 2.0_f64 + t3940 * t962 / 2.0_f64 + t3289 * t1265 + t1263 * t3293 + t960 * t3944 / 2.0_f64 + t422 * t10384 / 2.0_f64);
    (t10375, t10384, t10388)
}
