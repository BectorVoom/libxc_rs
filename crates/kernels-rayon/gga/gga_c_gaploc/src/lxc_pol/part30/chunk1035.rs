//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1035/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1035(t1508: f64, t1571: f64, t4416: f64, t4779: f64, t584: f64, t1461: f64, t1561: f64, t1397: f64, t4390: f64, t1238: f64, t4072: f64, t4081: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17551 = t1508 * t1571;
    let t17568 = t584 * t4779 * t4416;
    let t17571 = t1461 * t1561;
    let t18067 = t1397 * t4390;
    let t18089 = 1.0_f64 / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    (t17551, t17568, t17571, t18067, t18089, t18091)
}
