//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 845/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk845(t15772: f64, t167: f64, t569: f64, t2205: f64, t4454: f64, t616: f64, t12963: f64, t12965: f64, t12967: f64, t12975: f64, t17101: f64, t17104: f64, t17107: f64, t17111: f64, t17115: f64, t17120: f64, t17125: f64, t17129: f64, t17133: f64, t3281: f64, t446: f64) -> f64 {
    let t17137 = t569 * t167 * t15772;
    let t17141 = t2205 * t616 * t4454;
    let t17144 = 2.0_f64 / 3.0_f64 * t446 * t17101 - 2.0_f64 / 27.0_f64 * t17104 - 2.0_f64 / 3.0_f64 * t446 * t17107 - 2.0_f64 / 9.0_f64 * t446 * t17111 + 2.0_f64 / 3.0_f64 * t446 * t17115 + 2.0_f64 / 3.0_f64 * t446 * t17120 + 2.0_f64 / 3.0_f64 * t446 * t17125 - t12963 - t12965 - t12967 - t12975 - 4.0_f64 / 9.0_f64 * t3281 * t17129 - t446 * t17133 / 9.0_f64 - t446 * t17137 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t17141;
    t17144
}
