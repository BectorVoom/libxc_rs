//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 451/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk451(t1422: f64, t3278: f64, t3549: f64, t1423: f64, t3283: f64, t1173: f64, t459: f64, t1175: f64) -> (f64, f64, f64, f64) {
    let t3551 = t1422 * t3549 * t3278;
    let t3555 = t1422 * t1423 * t3283;
    let t3558 = t1173 * t459;
    let t3559 = t1175 * t1175;
    (t3551, t3555, t3558, t3559)
}
