//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 992/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk992(t14343: f64, t14396: f64, t14554: f64, t14598: f64, t1459: f64, t4513: f64, t9517: f64, t1555: f64, t524: f64, t1596: f64, t4348: f64, t4349: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t14600 = t14343 + t14396 + t14554 + t14598;
    let t14601 = t1459 * t14600;
    let t14602 = t9517 * t4513;
    let t14607 = t1555 * t1555;
    let t14608 = 1.0_f64 / t14607;
    let t14609 = t524 * t14608;
    let t14610 = t4348 * t1596;
    let t14612 = 1.0_f64 / t4349 / t544;
    (t14601, t14602, t14609, t14610, t14612)
}
