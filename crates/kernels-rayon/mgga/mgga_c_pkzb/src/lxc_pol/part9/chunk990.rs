//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 990/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk990(t12: f64, t7549: f64, t7896: f64, t1429: f64, t4803: f64, t1151: f64, t1153: f64, t2159: f64, t2163: f64, t3000: f64, t3005: f64, t318: f64, t319: f64, t808: f64, t810: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t7897 = t7549 + t7896;
    let t7906 = 2.0_f64 * t1429;
    let t7907 = 6.0_f64 * t4803;
    let t7908 = t7906 - t7907;
    let t7909 = piecewise3(t84, 0.0_f64, t7908);
    let t7913 = piecewise3(t203, 0.0_f64, t7897 * t319 / 2.0_f64 + t3000 * t810 + t1151 * t2163 / 2.0_f64 + t2159 * t1153 / 2.0_f64 + t808 * t3005 + t318 * t7909 / 2.0_f64);
    (t7897, t7908, t7909, t7913)
}
