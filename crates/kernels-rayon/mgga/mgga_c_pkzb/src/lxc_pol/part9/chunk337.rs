//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 337/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk337(t12: f64, t24: f64, t972: f64, t1151: f64, t318: f64, t319: f64, t201: f64, t977: f64, t326: f64, t1003: f64, t821: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t1153 = piecewise3(t84, 0.0_f64, t972);
    let t1157 = piecewise3(t203, 0.0_f64, t1151 * t319 / 2.0_f64 + t318 * t1153 / 2.0_f64);
    let t1158 = t201 * t1157;
    let t1161 = 1.0_f64 / t977;
    let t1162 = t326 * t1161;
    let t1165 = t821 * t1003;
    let t1167 = piecewise3(t90, 0.0_f64, -t1165 / 3.0_f64);
    (t1153, t1158, t1162, t1165, t1167)
}
