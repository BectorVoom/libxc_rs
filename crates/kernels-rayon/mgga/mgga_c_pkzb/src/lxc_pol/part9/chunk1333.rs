//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1333/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1333(t24: f64, t22149: f64, t1263: f64, t1265: f64, t22356: f64, t22386: f64, t22910: f64, t23547: f64, t23551: f64, t23554: f64, t23561: f64, t23567: f64, t2467: f64, t2471: f64, t3289: f64, t3293: f64, t422: f64, t423: f64, t6606: f64, t6613: f64, t8577: f64, t8587: f64, t960: f64, t962: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t23586 = piecewise3(t90, 0.0_f64, -t22149);
    let t23590 = piecewise3(t332, 0.0_f64, (t22356 + t22386 + t23547 + t23551 + t23554 + t23561 + t23567 + t22910) * t423 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t8577 * t962 + 3.0_f64 / 2.0_f64 * t3289 * t2471 + t1263 * t6613 / 2.0_f64 + t6606 * t1265 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2467 * t3293 + 3.0_f64 / 2.0_f64 * t960 * t8587 + t422 * t23586 / 2.0_f64);
    t23590
}
