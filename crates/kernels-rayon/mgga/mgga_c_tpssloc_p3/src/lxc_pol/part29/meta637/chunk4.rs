//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2095/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2095(t86942: f64, t23168: f64, t25338: f64, t13059: f64, t22979: f64, t25184: f64, t2713: f64, t2718: f64, t2742: f64, t4268: f64, t6627: f64, t7537: f64, t855: f64, t86929: f64, t86930: f64, t86931: f64, t86933: f64, t86941: f64) -> f64 {
    let t86943 = 0.38381794893125283518e-1_f64 * t86942;
    let t86950 = t23168 * t25338;
    let t86951 = 0.76763589786250567036e-1_f64 * t86950;
    let t86952 = -t86929 + t86930 - t86931 + 0.3289868133696452873e-1_f64 * t86933 + 2.0_f64 * t855 * t2718 * t7537 * t2742 + t86941 + t86943 + 4.0_f64 * t2713 * t25184 + 2.0_f64 * t6627 * t13059 + 4.0_f64 * t4268 * t22979 + t86951;
    t86952
}
