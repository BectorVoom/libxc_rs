//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 796/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk796(t776: f64, t11155: f64, t11162: f64, t12290: f64, t12306: f64, t1758: f64, t1995: f64, t4973: f64, t4977: f64, t525: f64, t5449: f64, t642: f64, t7567: f64, t773: f64) -> f64 {
    let t777 = t776 < -0.66725e-1_f64;
    let t12313 = piecewise3(t777, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t12290 * t642 - 10.0_f64 / 9.0_f64 * t525 * t5449 * t1758 + 40.0_f64 / 27.0_f64 * t525 * t1995 * t4973 - 10.0_f64 / 9.0_f64 * t525 * t1995 * t4977 - 280.0_f64 / 243.0_f64 * t525 * t773 * t11155 + 40.0_f64 / 27.0_f64 * t7567 * t12306 - 10.0_f64 / 27.0_f64 * t525 * t773 * t11162);
    t12313
}
