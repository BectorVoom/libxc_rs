//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1080/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1080(t29310: f64, t3887: f64, t1375: f64, t1843: f64, t24071: f64, t26184: f64, t26198: f64, t26200: f64, t26345: f64, t27009: f64, t27068: f64, t28118: f64, t28193: f64, t28196: f64, t28201: f64, t29287: f64, t29290: f64, t29293: f64, t29299: f64, t5321: f64, t568: f64, t7925: f64) -> (f64, f64) {
    let t29311 = t3887 * t29310;
    let t29314 = 0.15352717957250113407e0_f64 * t26184 + 0.3289868133696452873e-1_f64 * t26198 + t29287 * t568 + 0.76763589786250567036e-1_f64 * t26200 + 2.0_f64 * t29290 * t568 + t29293 * t568 - t24071 + 0.6579736267392905746e-1_f64 * t28118 - 2.0_f64 * t27068 * t1843 - 6.0_f64 * t1375 * t29299 + 0.16449340668482264365e-1_f64 * t26345 + 0.9869604401089358619e-1_f64 * t28193 - 0.3289868133696452873e-1_f64 * t28196 - 2.0_f64 * t27009 * t1843 + 0.16449340668482264365e-1_f64 * t28201 + 4.0_f64 * t5321 * t7925 + 4.0_f64 * t1375 * t29311;
    (t29311, t29314)
}
