//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 978/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk978(t5230: f64, t8465: f64, t8467: f64, t1814: f64, t31175: f64, t26288: f64, t5308: f64, t6950: f64, t3701: f64, t5187: f64, t26179: f64, t8327: f64) -> (f64, f64, f64, f64, f64) {
    let t120413 = t5230 * t8465 * t8467;
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    let t120669 = t3701 * t5187;
    let t120719 = 2.0_f64 * t26179 * t8327;
    (t120413, t120416, t120419, t120669, t120719)
}
