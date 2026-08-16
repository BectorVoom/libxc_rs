//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1010/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1010(t12: f64, t11113: f64, t11117: f64, t10518: f64, t1151: f64, t1153: f64, t318: f64, t319: f64, t3706: f64, t3710: f64, t201: f64, t199: f64, t399: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t11118 = t11113 + t11117;
    let t11125 = piecewise3(t84, 0.0_f64, t10518);
    let t11129 = piecewise3(t203, 0.0_f64, t11118 * t319 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t3706 * t1153 + 3.0_f64 / 2.0_f64 * t1151 * t3710 + t318 * t11125 / 2.0_f64);
    let t11130 = t201 * t11129;
    let t11131 = t199 * t11130;
    let t11132 = 0.2390625e-1_f64 * t11131;
    let t11133 = 1.0_f64 / t399;
    (t11118, t11125, t11130, t11132, t11133)
}
