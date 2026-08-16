//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1619/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1619(t248: f64, t3101: f64, t5867: f64, t1020: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t1046: f64, t13750: f64, t13758: f64, t13767: f64, t13946: f64, t17593: f64, t17596: f64, t17599: f64, t17602: f64, t17607: f64, t973: f64) -> (f64, f64, f64) {
    let t17611 = t248 * t3101 * t5867;
    let t17612 = t1020 * t17611;
    let t17614 = -t973 * t17593 / 144.0_f64 + t973 * t17596 / 216.0_f64 + t973 * t17599 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t973 * t17602 - t13750 + t10372 / 2592.0_f64 + t10377 + t10381 / 162.0_f64 + t10385 + t17607 * t1046 / 4608.0_f64 + t13758 + t13767 - t13946 + t17612 / 4608.0_f64;
    (t17611, t17612, t17614)
}
