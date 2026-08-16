//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1024/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1024(t128570: f64, t2020: f64, t127114: f64, t1983: f64, t2095: f64, t115925: f64, t28831: f64, t33363: f64, t7756: f64, t33623: f64, t7685: f64, t101138: f64, t26161: f64, t33221: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128571 = t128570 * t2020;
    let t128573 = t1983 * t2095 * t127114;
    let t128575 = 6.0_f64 * t115925 * t28831;
    let t128577 = 2.0_f64 * t33363 * t7756;
    let t128581 = 2.0_f64 * t7685 * t33623;
    let t128584 = 4.0_f64 * t26161 * t101138 * t33221;
    (t128571, t128573, t128575, t128577, t128581, t128584)
}
