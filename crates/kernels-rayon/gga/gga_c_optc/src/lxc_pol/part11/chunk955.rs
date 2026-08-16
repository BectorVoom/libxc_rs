//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 955/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk955(t11928: f64, t1220: f64, t15012: f64, t15099: f64, t15101: f64, t15105: f64, t15108: f64, t15138: f64, t1554: f64, t1570: f64, t1579: f64, t17465: f64, t17471: f64, t17504: f64, t17516: f64, t17527: f64, t17531: f64, t17536: f64, t17543: f64, t3980: f64, t4230: f64, t4297: f64, t4536: f64, t5098: f64, t5103: f64, t5441: f64) -> f64 {
    let t17548 = -4.0_f64 / 3.0_f64 * t1220 * t17465 + t17471 - t17504 + t15012 * t1579 / 2.0_f64 + t4536 * t5098 / 2.0_f64 + 2.0_f64 / 3.0_f64 * t4536 * t5103 - 4.0_f64 / 3.0_f64 * t4230 * t5098 - 16.0_f64 / 9.0_f64 * t4230 * t5103 - 100.0_f64 / 27.0_f64 * t4297 * t17516 - t11928 / 9.0_f64 - 0.77534644304710291488e-2_f64 * t3980 * t15138 * t1554 + 100.0_f64 / 27.0_f64 * t15099 - 50.0_f64 / 3.0_f64 * t15101 + 20000.0_f64 / 81.0_f64 * t15105 - t17527 - t17531 + 100.0_f64 / 81.0_f64 * t15108 + 4000000.0_f64 / 243.0_f64 * t17536 * t17543 + 44.0_f64 / 3.0_f64 * t1570 * t5441;
    t17548
}
