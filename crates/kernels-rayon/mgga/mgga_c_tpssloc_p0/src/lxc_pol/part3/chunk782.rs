//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 782/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk782(t4433: f64, t932: f64, t1568: f64, t2888: f64, t931: f64, t2766: f64, t2892: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t324: f64) -> (f64, f64, f64, f64, f64) {
    let t4434 = t4433 * t932;
    let t4437 = t1568 * t2888;
    let t4438 = t4437 * t931;
    let t4446 = t2892 + 0.30902777777777777778e-2_f64 * t2766 + 0.30902777777777777778e-2_f64 * t4335 - 0.61805555555555555555e-2_f64 * t4340 + 0.18541666666666666667e-1_f64 * t4345 - 0.92708333333333333333e-2_f64 * t4349;
    let t4447 = t4446 * t324;
    (t4434, t4437, t4438, t4446, t4447)
}
